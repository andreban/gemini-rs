use serde::{Deserialize, Serialize};

use super::Content;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    pub contents: Content,
}

impl CountTokensRequest {
    pub fn builder() -> CountTokensRequestBuilder {
        CountTokensRequestBuilder::new()
    }
}

pub struct CountTokensRequestBuilder {
    contents: Content,
}

impl CountTokensRequestBuilder {
    pub fn new() -> Self {
        CountTokensRequestBuilder {
            contents: Content::default(),
        }
    }

    pub fn from_prompt(prompt: &str) -> Self {
        CountTokensRequestBuilder {
            contents: Content {
                parts: Some(vec![super::Part::Text(prompt.to_string())]),
                ..Default::default()
            },
        }
    }

    pub fn build(self) -> CountTokensRequest {
        CountTokensRequest {
            contents: self.contents,
        }
    }
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
        error: super::VertexApiError,
    },
}
