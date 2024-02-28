use serde::{Deserialize, Serialize};

use super::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEmbeddingRequest {
    pub instances: Vec<TextEmbeddingRequestInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEmbeddingRequestInstance {
    pub content: String,
    pub task_type: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextEmbeddingResponse {
    Ok {
        predictions: Vec<TextEmbeddingPrediction>,
    },
    Error {
        error: Error,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEmbeddingPrediction {
    pub embeddings: TextEmbeddingResult,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEmbeddingResult {
    statistics: TextEmbeddingStatistics,
    values: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEmbeddingStatistics {
    truncated: bool,
    token_count: u32,
}
