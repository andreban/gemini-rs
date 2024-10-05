use gemini_rs::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let authentication_manager = gcp_auth::provider().await?;
    let api_endpoint = std::env::var("API_ENDPOINT")?;
    let project_id = std::env::var("PROJECT_ID")?;
    let location_id = std::env::var("LOCATION_ID")?;

    let gemini = GeminiClient::new(
        authentication_manager,
        api_endpoint,
        project_id,
        location_id,
    );

    let prompt = "Generate 10 ideas of blog posts with a title and decription for each idea.";
    let request = GenerateContentRequest {
        contents: vec![Content {
            role: Some("user".to_string()),
            parts: Some(vec![Part::Text(prompt.to_string())]),
        }],
        generation_config: Some(GenerationConfig {
            response_mime_type: Some("application/json".to_string()),
            response_schema: Some(json!({
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "title": {
                            "type": "STRING"
                        },
                        "description": {
                            "type": "STRING"
                        }
                    }
                }
            })),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = gemini
        .generate_content(&request, "gemini-1.5-flash-001")
        .await?;

    println!("Response: {:?}", result.candidates[0].get_text().unwrap());

    Ok(())
}
