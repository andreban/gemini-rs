use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Content, Error, Part};

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
#[serde(rename_all = "camelCase")]
pub struct Citation {
    pub start_index: Option<i32>,
    pub end_index: Option<i32>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
    pub probability_score: f32,
    pub severity: String,
    pub severity_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub candidates_token_count: Option<u32>,
    pub prompt_token_count: u32,
    pub total_token_count: u32,
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

#[cfg(test)]
mod tests {
    use super::GenerateContentResponse;

    #[test]
    pub fn parses_max_tokens_response() {
        let input = r#"{
            "candidates": [
              {
                "content": {
                  "role": "model",
                  "parts": [
                    {
                      "text": "Service workers are powerful and absolutely worth learning. They let you deliver an entirely new level of experience to your users. Your site can load instantly . It can work offline . It can be installed as a platform-specific app and feel every bit as polished—but with the reach and freedom of the web."
                    }
                  ]
                },
                "finishReason": "MAX_TOKENS",
                "safetyRatings": [
                  {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "probability": "NEGLIGIBLE",
                    "probabilityScore": 0.03882902,
                    "severity": "HARM_SEVERITY_NEGLIGIBLE",
                    "severityScore": 0.05781161
                  },
                  {
                    "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                    "probability": "NEGLIGIBLE",
                    "probabilityScore": 0.07626997,
                    "severity": "HARM_SEVERITY_NEGLIGIBLE",
                    "severityScore": 0.06705628
                  },
                  {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "probability": "NEGLIGIBLE",
                    "probabilityScore": 0.05749328,
                    "severity": "HARM_SEVERITY_NEGLIGIBLE",
                    "severityScore": 0.027532939
                  },
                  {
                    "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                    "probability": "NEGLIGIBLE",
                    "probabilityScore": 0.12929276,
                    "severity": "HARM_SEVERITY_NEGLIGIBLE",
                    "severityScore": 0.17838266
                  }
                ],
                "citationMetadata": {
                  "citations": [
                    {
                      "endIndex": 151,
                      "uri": "https://web.dev/service-worker-mindset/"
                    },
                    {
                      "startIndex": 93,
                      "endIndex": 297,
                      "uri": "https://web.dev/service-worker-mindset/"
                    },
                    {
                      "endIndex": 297
                    }
                  ]
                }
              }
            ],
            "usageMetadata": {
              "promptTokenCount": 12069,
              "candidatesTokenCount": 61,
              "totalTokenCount": 12130
            }
          }"#;
        serde_json::from_str::<GenerateContentResponse>(input).unwrap();
    }
}
