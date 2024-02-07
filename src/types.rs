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

#[derive(Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
    pub tools: Option<Vec<Tools>>,
}

#[derive(Serialize, Deserialize)]
pub struct Tools {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Option<Vec<Part>>,
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

#[derive(Debug, Serialize, Deserialize)]
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

pub type GenerateContentResponse = Vec<ResponseStreamChunk>;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[serde(untagged)]
// pub enum ResponseStreamChunkType {
//     Ok {
//         candidates: Vec<Candidate>,
//         usage_metadata: UsageMetadata,
//     },
//     Error,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStreamChunk {
    pub candidates: Option<Vec<Candidate>>,
    pub usage_metadata: Option<UsageMetadata>,
    pub error: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub citation_metadata: Option<CitationMetadata>,
    pub safety_ratings: Vec<SafetyRating>,
    pub finish_reason: Option<String>,
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
    pub uri: String,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameters {
    pub r#type: String,
    pub properties: HashMap<String, FunctionParametersProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
