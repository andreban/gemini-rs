use std::vec;

use gemini_rs::prelude::*;

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

    let prompt = "What is the airspeed of an unladen swallow?";

    let request = GenerateContentRequest {
        contents: vec![Content {
            role: Some(Role::User),
            parts: Some(vec![Part::Text(prompt.to_string())]),
        }],
        safety_settings: Some(vec![SafetySetting {
            category: HarmCategory::HateSpeech,
            threshold: HarmBlockThreshold::BlockNone,
            method: None,
        }]),
        ..Default::default()
    };

    println!("{}", serde_json::to_string_pretty(&request).unwrap());

    let result = gemini
        .generate_content(&request, "gemini-1.0-pro-002")
        .await?;
    println!("Response: {:?}", result);

    Ok(())
}
