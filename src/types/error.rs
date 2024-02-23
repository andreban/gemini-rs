use serde::{Deserialize, Serialize};

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
