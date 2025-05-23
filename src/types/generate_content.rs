use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Content, VertexApiError};
use crate::error::Result;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tools>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
}

impl GenerateContentRequest {
    pub fn builder() -> GenerateContentRequestBuilder {
        GenerateContentRequestBuilder::new()
    }
}

pub struct GenerateContentRequestBuilder {
    request: GenerateContentRequest,
}

impl GenerateContentRequestBuilder {
    fn new() -> Self {
        GenerateContentRequestBuilder {
            request: GenerateContentRequest::default(),
        }
    }

    pub fn contents(mut self, contents: Vec<Content>) -> Self {
        self.request.contents = contents;
        self
    }

    pub fn generation_config(mut self, generation_config: GenerationConfig) -> Self {
        self.request.generation_config = Some(generation_config);
        self
    }

    pub fn tools(mut self, tools: Vec<Tools>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    pub fn safety_settings(mut self, safety_settings: Vec<SafetySetting>) -> Self {
        self.request.safety_settings = Some(safety_settings);
        self
    }

    pub fn system_instruction(mut self, system_instruction: Content) -> Self {
        self.request.system_instruction = Some(system_instruction);
        self
    }

    pub fn build(self) -> GenerateContentRequest {
        self.request
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Tools {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,

    #[serde(rename = "googleSearchRetrieval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GoogleSearch>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GoogleSearch {}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRetrievalConfig {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_threshold: Option<f32>,
}

impl Default for DynamicRetrievalConfig {
    fn default() -> Self {
        Self {
            mode: "MODE_DYNAMIC".to_string(),
            dynamic_threshold: Some(0.7),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchRetrieval {
    pub dynamic_retrieval_config: DynamicRetrievalConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<Value>,
}

impl GenerationConfig {
    pub fn builder() -> GenerationConfigBuilder {
        GenerationConfigBuilder::new()
    }
}

pub struct GenerationConfigBuilder {
    generation_config: GenerationConfig,
}

impl GenerationConfigBuilder {
    fn new() -> Self {
        Self {
            generation_config: Default::default(),
        }
    }

    pub fn max_output_tokens<T: Into<i32>>(mut self, max_output_tokens: T) -> Self {
        self.generation_config.max_output_tokens = Some(max_output_tokens.into());
        self
    }

    pub fn temperature<T: Into<f32>>(mut self, temperature: T) -> Self {
        self.generation_config.temperature = Some(temperature.into());
        self
    }

    pub fn top_p<T: Into<f32>>(mut self, top_p: T) -> Self {
        self.generation_config.top_p = Some(top_p.into());
        self
    }

    pub fn top_k<T: Into<i32>>(mut self, top_k: T) -> Self {
        self.generation_config.top_k = Some(top_k.into());
        self
    }

    pub fn stop_sequences<T: Into<Vec<String>>>(mut self, stop_sequences: T) -> Self {
        self.generation_config.stop_sequences = Some(stop_sequences.into());
        self
    }

    pub fn candidate_count<T: Into<u32>>(mut self, candidate_count: T) -> Self {
        self.generation_config.candidate_count = Some(candidate_count.into());
        self
    }

    pub fn response_mime_type<T: Into<String>>(mut self, response_mime_type: T) -> Self {
        self.generation_config.response_mime_type = Some(response_mime_type.into());
        self
    }

    pub fn response_schema<T: Into<Value>>(mut self, response_schema: T) -> Self {
        self.generation_config.response_schema = Some(response_schema.into());
        self
    }

    pub fn build(self) -> GenerationConfig {
        self.generation_config
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HarmBlockMethod>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HateSpeech,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    DangerousContent,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    Harassment,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    SexuallyExplicit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
    BlockNone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HarmBlockMethod {
    #[serde(rename = "HARM_BLOCK_METHOD_UNSPECIFIED")]
    Unspecified, // HARM_BLOCK_METHOD_UNSPECIFIED
    #[serde(rename = "SEVERITY")]
    Severity, // SEVERITY
    #[serde(rename = "PROBABILITY")]
    Probability, // PROBABILITY
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

impl Candidate {
    pub fn get_text(&self) -> Option<String> {
        match &self.content {
            Some(content) => content.get_text(),
            None => None,
        }
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
    pub probability_score: Option<f32>,
    pub severity: String,
    pub severity_score: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub candidates_token_count: Option<u32>,
    pub prompt_token_count: Option<u32>,
    pub total_token_count: Option<u32>,
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
#[serde(untagged)]
pub enum GenerateContentResponse {
    Ok(GenerateContentResponseResult),
    Error(GenerateContentResponseError),
}

impl From<GenerateContentResponse> for Result<GenerateContentResponseResult> {
    fn from(val: GenerateContentResponse) -> Self {
        match val {
            GenerateContentResponse::Ok(result) => Ok(result),
            GenerateContentResponse::Error(error) => Err(error.error.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponseResult {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateContentResponseError {
    pub error: VertexApiError,
}

impl GenerateContentResponse {
    pub fn into_result(self) -> Result<GenerateContentResponseResult> {
        match self {
            GenerateContentResponse::Ok(result) => Ok(result),
            GenerateContentResponse::Error(error) => Err(error.error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GenerateContentResponse, GenerateContentResponseResult};

    #[test]
    pub fn parses_empty_metadata_response() {
        let input = r#"{"candidates": [{"content": {"role": "model","parts": [{"text": "-"}]}}],"usageMetadata": {}}"#;
        serde_json::from_str::<GenerateContentResponseResult>(input).unwrap();
    }

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

    #[test]
    fn parses_candidates_without_content() {
        let input = r#"{
        "candidates": [
          {
            "finishReason": "RECITATION",
            "safetyRatings": [
              {
                "category": "HARM_CATEGORY_HATE_SPEECH",
                "probability": "NEGLIGIBLE",
                "probabilityScore": 0.08021325,
                "severity": "HARM_SEVERITY_NEGLIGIBLE",
                "severityScore": 0.0721122
              },
              {
                "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                "probability": "NEGLIGIBLE",
                "probabilityScore": 0.19360436,
                "severity": "HARM_SEVERITY_NEGLIGIBLE",
                "severityScore": 0.1066906
              },
              {
                "category": "HARM_CATEGORY_HARASSMENT",
                "probability": "NEGLIGIBLE",
                "probabilityScore": 0.07751766,
                "severity": "HARM_SEVERITY_NEGLIGIBLE",
                "severityScore": 0.040769264
              },
              {
                "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                "probability": "NEGLIGIBLE",
                "probabilityScore": 0.030792166,
                "severity": "HARM_SEVERITY_NEGLIGIBLE",
                "severityScore": 0.04138472
              }
            ],
            "citationMetadata": {
              "citations": [
                {
                  "startIndex": 1108,
                  "endIndex": 1250,
                  "uri": "https://chrome.google.com/webstore/detail/autocontrol-shortcut-mana/lkaihdpfpifdlgoapbfocpmekbokmcfd?hl=zh-TW"
                }
              ]
            }
          }
        ],
        "usageMetadata": {
          "promptTokenCount": 577,
          "totalTokenCount": 577
        }
      }"#;
        serde_json::from_str::<GenerateContentResponse>(input).unwrap();
    }

    #[test]
    fn parses_safety_rating_without_scores() {
        let input = r#"{
          "candidates": [
            {
              "content": {
                "role": "model",
                "parts": [
                  {
                    "text": "Return text"
                  }
                ]
              },
              "finishReason": "STOP",
              "safetyRatings": [
                {
                  "category": "HARM_CATEGORY_HATE_SPEECH",
                  "probability": "NEGLIGIBLE",
                  "severity": "HARM_SEVERITY_NEGLIGIBLE"
                },
                {
                  "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                  "probability": "NEGLIGIBLE",
                  "severity": "HARM_SEVERITY_NEGLIGIBLE"
                },
                {
                  "category": "HARM_CATEGORY_HARASSMENT",
                  "probability": "NEGLIGIBLE",
                  "severity": "HARM_SEVERITY_NEGLIGIBLE"
                },
                {
                  "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                  "probability": "NEGLIGIBLE",
                  "severity": "HARM_SEVERITY_NEGLIGIBLE"
                }
              ]
            }
          ],
          "usageMetadata": {
            "promptTokenCount": 5492,
            "candidatesTokenCount": 1256,
            "totalTokenCount": 6748
          }
        }"#;
        serde_json::from_str::<GenerateContentResponse>(input).unwrap();
    }
}
