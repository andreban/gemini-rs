use serde::{Deserialize, Serialize};

use super::Content;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    pub contents: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CountTokensResponse {
    #[serde(rename_all = "camelCase")]
    Ok {
        total_tokens: i32,
        total_billable_characters: u32,
    },
    Error {
        error: super::Error,
    },
}
