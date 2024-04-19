use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub status: String,
    pub details: Option<Vec<ErrorType>>,
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

    #[serde(rename = "type.googleapis.com/google.rpc.BadRequest")]
    BadRequest {
        #[serde(rename = "fieldViolations")]
        field_violations: Vec<FieldViolation>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorInfoMetadata {
    pub service: String,
    pub consumer: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldViolation {
    pub field: String,
    pub description: String,
}
