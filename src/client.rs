use std::sync::Arc;

use deadqueue::unlimited::Queue;
use futures_util::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

use crate::dialogue::{Message, Role};
use crate::error::{Error, Result};
use crate::prelude::{
    Candidate, Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig,
    TextEmbeddingRequest, TextEmbeddingResponse,
};
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

    pub async fn stream_generate_content(
        &self,
        request: &GenerateContentRequest,
        model: &str,
    ) -> Arc<Queue<Option<GenerateContentResponse>>> {
        let queue = Arc::new(Queue::<Option<GenerateContentResponse>>::new());

        // Clone the queue and other necessary data to move into the async block.
        let cloned_queue = queue.clone();
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await.unwrap();
        let endpoint_url: String = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent?alt=sse", self.api_endpoint, self.project_id, self.location_id, model.to_string(),
        );
        let client = self.client.clone();
        let request = request.clone();

        // Start a thread to run the request in the background.
        tokio::spawn(async move {
            let req = client
                .post(&endpoint_url)
                .bearer_auth(access_token)
                .json(&request);
            let mut event_source = EventSource::new(req).unwrap();
            while let Some(Ok(event)) = event_source.next().await {
                if let Event::Message(event) = event {
                    let response: GenerateContentResponse =
                        serde_json::from_str(&event.data).unwrap();
                    cloned_queue.push(Some(response));
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
    ) -> Result<GenerateContentResponse> {
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let endpoint_url: String = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:generateContent", self.api_endpoint, self.project_id, self.location_id, model.to_string(),
        );
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let txt_json = resp.text().await?;
        match serde_json::from_str(&txt_json) {
            Ok(response) => Ok(response),
            Err(e) => {
                tracing::error!("Failed to parse response: {} with error {}", txt_json, e);
                Err(e.into())
            }
        }
    }

    /// Prompts a conversation to the model.
    pub async fn prompt_conversation(&self, messages: &[Message]) -> Result<Message> {
        let request = GenerateContentRequest {
            contents: messages
                .iter()
                .map(|m| Content {
                    role: m.role.to_string(),
                    parts: Some(vec![Part::Text(m.text.clone())]),
                })
                .collect(),
            generation_config: None,
            tools: None,
        };

        let response = self.generate_content(&request, "gemini-pro").await?;

        // Check for errors in the response.
        let mut candidates = GeminiClient::<T>::collect_text_from_response(&response)?;

        match candidates.pop() {
            Some(text) => Ok(Message::new(Role::Model, &text)),
            None => Err(Error::NoCandidatesError),
        }
    }

    /// Sends a text prompt to the Vertex API using the Gemini Pro model and extracts the text
    /// from the response.
    pub async fn prompt_text(
        &self,
        prompt: &str,
        generation_config: Option<&GenerationConfig>,
    ) -> Result<String> {
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: Some(vec![Part::Text(prompt.to_string())]),
            }],
            generation_config: generation_config.cloned(),
            tools: None,
        };

        let response = self.generate_content(&request, "gemini-pro").await?;
        let mut candidates = GeminiClient::<T>::collect_text_from_response(&response)?;

        match candidates.pop() {
            Some(candidate) => Ok(candidate),
            None => Err(Error::NoCandidatesError),
        }
    }

    fn collect_text_from_response(response: &GenerateContentResponse) -> Result<Vec<String>> {
        match response {
            GenerateContentResponse::Ok {
                candidates,
                usage_metadata: _,
            } => Ok(candidates
                .iter()
                .map(Candidate::get_text)
                .flatten()
                .collect::<Vec<String>>()),
            GenerateContentResponse::Error { error } => {
                tracing::error!("Error in response: {:?}", error);
                return Err(Error::VertexError(error.clone()));
            }
        }
    }

    pub async fn text_embeddings(
        &self,
        request: &TextEmbeddingRequest,
    ) -> Result<TextEmbeddingResponse> {
        let model = "textembedding-gecko@003";
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
        println!("{}", txt_json);
        Ok(serde_json::from_str::<TextEmbeddingResponse>(&txt_json)?)
    }
}
