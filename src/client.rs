use crate::error::Result as GeminiResult;
use std::sync::Arc;
use std::vec;
use tokio_stream::{Stream, StreamExt};

use deadqueue::unlimited::Queue;
use reqwest_eventsource::{Event, EventSource};
use tracing::error;

use crate::dialogue::Message;
use crate::error::{Error, Result};
use crate::prelude::{
    Candidate, Content, CountTokensRequest, CountTokensResponse, GenerateContentRequest,
    GenerateContentResponse, GenerateContentResponseResult, TextEmbeddingRequest,
    TextEmbeddingResponse,
};
use crate::types::{PredictImageRequest, PredictImageResponse, Role};
use crate::{prelude::Part, token_provider::TokenProvider};

pub static AUTH_SCOPE: &[&str] = &["https://www.googleapis.com/auth/cloud-platform"];

#[derive(Clone, Debug)]
pub struct GeminiClient<T: TokenProvider + Clone> {
    token_provider: T,
    client: reqwest::Client,
    api_endpoint: String,
    project_id: String,
    location_id: String,
}

unsafe impl<T: TokenProvider + Clone> Send for GeminiClient<T> {}
unsafe impl<T: TokenProvider + Clone> Sync for GeminiClient<T> {}

impl<T: TokenProvider + Clone> GeminiClient<T> {
    pub fn new(
        token_provider: T,
        api_endpoint: String,
        project_id: String,
        location_id: String,
    ) -> Self {
        GeminiClient {
            token_provider,
            client: reqwest::Client::new(),
            api_endpoint,
            project_id,
            location_id,
        }
    }

    pub async fn generate_content_stream(
        &self,
        request: &GenerateContentRequest,
        model: &str,
    ) -> Result<impl Stream<Item = GeminiResult<GenerateContentResponseResult>>> {
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await.unwrap();
        let endpoint_url = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent?alt=sse", self.api_endpoint, self.project_id, self.location_id, model,
        );
        let client = self.client.clone();
        let request = request.clone();
        let req = client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request);

        let event_source = EventSource::new(req).unwrap();

        let mapped = event_source.filter_map(|event| {
            let event = match event {
                Ok(event) => event,
                Err(e) => return Some(Err(e.into())),
            };

            let Event::Message(event_message) = event else {
                return None;
            };

            let gemini_response: GenerateContentResponse =
                match serde_json::from_str(&event_message.data) {
                    Ok(gemini_response) => gemini_response,
                    Err(e) => return Some(Err(e.into())),
                };

            let gemini_response = match gemini_response.into_result() {
                Ok(gemini_response) => gemini_response,
                Err(e) => return Some(Err(e)),
            };

            Some(Ok(gemini_response))
        });
        Ok(mapped)
    }

    pub async fn stream_generate_content(
        &self,
        request: &GenerateContentRequest,
        model: &str,
    ) -> Arc<Queue<Option<Result<GenerateContentResponseResult>>>> {
        let queue = Arc::new(Queue::<Option<Result<GenerateContentResponseResult>>>::new());
        let access_token = match self.token_provider.get_token(AUTH_SCOPE).await {
            Ok(access_token) => access_token,
            Err(e) => {
                queue.push(Some(Err(e)));
                return queue;
            }
        };

        // Clone the queue and other necessary data to move into the async block.
        let cloned_queue = queue.clone();
        let endpoint_url: String = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent?alt=sse", self.api_endpoint, self.project_id, self.location_id, model,
        );
        let client = self.client.clone();
        let request = request.clone();

        // Start a thread to run the request in the background.
        tokio::spawn(async move {
            let req = client
                .post(&endpoint_url)
                .bearer_auth(access_token)
                .json(&request);

            let mut event_source = match EventSource::new(req) {
                Ok(event_source) => event_source,
                Err(e) => {
                    cloned_queue.push(Some(Err(e.into())));
                    return;
                }
            };
            while let Some(event) = event_source.next().await {
                match event {
                    Ok(event) => {
                        if let Event::Message(event) = event {
                            let response: serde_json::error::Result<GenerateContentResponse> =
                                serde_json::from_str(&event.data);

                            match response {
                                Ok(response) => {
                                    let result = response.into_result();
                                    let finished = match &result {
                                        Ok(result) => result.candidates[0].finish_reason.is_some(),
                                        Err(_) => true,
                                    };
                                    cloned_queue.push(Some(result));
                                    if finished {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    tracing::error!("Error parsing message: {}", event.data);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error in event source: {:?}", e);
                        break;
                    }
                }
            }
            cloned_queue.push(None);
        });

        // Return the queue that will receive the responses.
        queue
    }

    pub async fn generate_content(
        &self,
        request: &GenerateContentRequest,
        model: &str,
    ) -> Result<GenerateContentResponseResult> {
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let endpoint_url: String = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:generateContent", self.api_endpoint, self.project_id, self.location_id, model,
        );
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let txt_json = resp.text().await?;
        tracing::debug!("generate_content response: {:?}", txt_json);
        match serde_json::from_str::<GenerateContentResponse>(&txt_json) {
            Ok(response) => Ok(response.into_result()?),
            Err(e) => {
                tracing::error!("Failed to parse response: {} with error {}", txt_json, e);
                Err(e.into())
            }
        }
    }

    /// Prompts a conversation to the model.
    pub async fn prompt_conversation(&self, messages: &[Message], model: &str) -> Result<Message> {
        let request = GenerateContentRequest {
            contents: messages
                .iter()
                .map(|m| Content {
                    role: Some(m.role),
                    parts: Some(vec![Part::Text(m.text.clone())]),
                })
                .collect(),
            generation_config: None,
            tools: None,
            system_instruction: None,
            safety_settings: None,
        };

        let response = self.generate_content(&request, model).await?;

        // Check for errors in the response.
        let mut candidates = GeminiClient::<T>::collect_text_from_response(&response);

        match candidates.pop() {
            Some(text) => Ok(Message::new(Role::Model, &text)),
            None => Err(Error::NoCandidatesError),
        }
    }

    fn collect_text_from_response(response: &GenerateContentResponseResult) -> Vec<String> {
        response
            .candidates
            .iter()
            .filter_map(Candidate::get_text)
            .collect::<Vec<String>>()
    }

    pub async fn text_embeddings(
        &self,
        request: &TextEmbeddingRequest,
        model: &str,
    ) -> Result<TextEmbeddingResponse> {
        let endpoint_url = format!(
            "https://{}/v1/projects/{}/locations/{}/publishers/google/models/{}:predict",
            self.api_endpoint, self.project_id, self.location_id, model,
        );
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;
        let txt_json = resp.text().await?;
        tracing::debug!("text_embeddings response: {:?}", txt_json);
        Ok(serde_json::from_str::<TextEmbeddingResponse>(&txt_json)?)
    }

    pub async fn count_tokens(
        &self,
        request: &CountTokensRequest,
        model: &str,
    ) -> Result<CountTokensResponse> {
        let endpoint_url = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:countTokens",
            self.api_endpoint, self.project_id, self.location_id, model,
        );
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let txt_json = resp.text().await?;
        tracing::debug!("count_tokens response: {:?}", txt_json);
        Ok(serde_json::from_str(&txt_json)?)
    }

    pub async fn predict_image(
        &self,
        request: &PredictImageRequest,
        model: &str,
    ) -> Result<PredictImageResponse> {
        let endpoint_url = format!(
            "https://{}/v1/projects/{}/locations/{}/publishers/google/models/{}:predict",
            self.api_endpoint, self.project_id, self.location_id, model,
        );

        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let txt_json = resp.text().await?;

        match serde_json::from_str::<PredictImageResponse>(&txt_json) {
            Ok(response) => Ok(response),
            Err(e) => {
                error!(response = txt_json, error = ?e, "Failed to parse response");
                Err(e.into())
            }
        }
    }
}
