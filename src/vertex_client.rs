use crate::conversation::{Message, Role};
use crate::error::Result;
use crate::prelude::{
    Content, Conversation, GenerateContentRequest, GenerateContentResponse, GenerationConfig,
};
use crate::{prelude::Part, token_provider::TokenProvider};

pub static AUTH_SCOPE: &[&str] = &["https://www.googleapis.com/auth/cloud-platform"];

pub enum Model {
    GeminiPro,
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::GeminiPro => "gemini-pro".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct VertexClient<T: TokenProvider + Clone> {
    token_provider: T,
    client: reqwest::Client,
    api_endpoint: String,
    project_id: String,
    location_id: String,
}

unsafe impl<T: TokenProvider + Clone> Send for VertexClient<T> {}
unsafe impl<T: TokenProvider + Clone> Sync for VertexClient<T> {}

impl<T: TokenProvider + Clone> VertexClient<T> {
    pub fn new(
        token_provider: T,
        api_endpoint: String,
        project_id: String,
        location_id: String,
    ) -> Self {
        VertexClient {
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
        model: Model,
    ) -> Result<GenerateContentResponse> {
        let access_token = self.token_provider.get_token(AUTH_SCOPE).await?;
        let endpoint_url: String = format!(
            "https://{}/v1beta1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent", self.api_endpoint, self.project_id, self.location_id, model.to_string(),
        );
        let resp = self
            .client
            .post(&endpoint_url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let txt_json = resp.text().await?;
        tracing::debug!("Vertex API Response: {}", txt_json);
        match serde_json::from_str(&txt_json) {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Failed to parse response: {} / {}", txt_json, e);
                Err(e.into())
            }
        }
    }

    /// Prompts a conversation to the model.
    pub async fn prompt_conversation(&self, conversation: &Conversation) -> Result<Message> {
        let request = GenerateContentRequest {
            contents: conversation
                .messages
                .iter()
                .map(|m| Content {
                    role: m.role.to_string(),
                    parts: vec![Part::Text(m.text.clone())],
                })
                .collect(),
            generation_config: None,
            tools: None,
        };

        let response = self
            .stream_generate_content(&request, Model::GeminiPro)
            .await?;

        let text = response
            .0
            .into_iter()
            .flat_map(|chunk| {
                chunk.candidates.unwrap().into_iter().flat_map(|candidate| {
                    candidate
                        .content
                        .parts
                        .into_iter()
                        .map(|part| match part {
                            Part::Text(text) => Some(text),
                            _ => None,
                        })
                        .filter(Option::is_some)
                        .flatten()
                })
            })
            .collect::<String>();
        Ok(Message::new(Role::Model, &text))
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
                parts: vec![Part::Text(prompt.to_string())],
            }],
            generation_config: generation_config.cloned(),
            tools: None,
        };

        let response = self
            .stream_generate_content(&request, Model::GeminiPro)
            .await?;

        let text = response
            .0
            .into_iter()
            .flat_map(|chunk| {
                chunk.candidates.unwrap().into_iter().flat_map(|candidate| {
                    candidate
                        .content
                        .parts
                        .into_iter()
                        .map(|part| match part {
                            Part::Text(text) => Some(text),
                            _ => None,
                        })
                        .filter(Option::is_some)
                        .flatten()
                })
            })
            .collect::<String>();
        Ok(text)
    }
}
