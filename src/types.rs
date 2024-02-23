use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    pub contents: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
    pub tools: Option<Vec<Tools>>,
}

impl GenerateContentRequest {
    pub fn from_prompt(prompt: &str, generation_config: Option<GenerationConfig>) -> Self {
        GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: Some(vec![Part::Text(prompt.to_string())]),
            }],
            generation_config,
            tools: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tools {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Option<Vec<Part>>,
}

impl Content {
    pub fn get_text(&self) -> Option<String> {
        self.parts.as_ref().map(|parts| {
            parts
                .iter()
                .filter_map(|part| match part {
                    Part::Text(text) => Some(text.clone()),
                    _ => None,
                })
                .collect::<String>()
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub max_output_tokens: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub stop_sequences: Option<Vec<String>>,
    pub candidate_count: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Part {
    Text(String),
    InlineData {
        mime_type: String,
        data: String,
    },
    FileData {
        mime_type: String,
        file_uri: String,
    },
    FunctionCall {
        name: String,
        args: HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum GenerateContentResponse {
    Ok {
        candidates: Vec<Candidate>,
        usage_metadata: Option<UsageMetadata>,
    },
    Error {
        error: Error,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub citation_metadata: Option<CitationMetadata>,
    pub safety_ratings: Vec<SafetyRating>,
    pub finish_reason: Option<String>,
}

impl Candidate {
    pub fn get_text(&self) -> Option<String> {
        self.content.get_text()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    pub start_index: i32,
    pub end_index: i32,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub candidates_token_count: Option<i32>,
    pub prompt_token_count: i32,
    pub total_token_count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameters {
    pub r#type: String,
    pub properties: HashMap<String, FunctionParametersProperty>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParametersProperty {
    pub r#type: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub status: String,
    pub details: Vec<ErrorType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub description: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum ErrorType {
    #[serde(rename = "type.googleapis.com/google.rpc.ErrorInfo")]
    ErrorInfo { metadata: ErrorInfoMetadata },

    #[serde(rename = "type.googleapis.com/google.rpc.Help")]
    Help { links: Vec<Link> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorInfoMetadata {
    service: String,
    consumer: String,
}
