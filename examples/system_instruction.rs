use gemini_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
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

    let system_instruction = "Answer as if you were Winston Churchill";
    let prompt = "What is the airspeed of an unladen swallow?";

    let request = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: Some(vec![Part::Text(prompt.to_string())]),
        }],
        system_instruction: Some(Content {
            role: "system".to_string(),
            parts: Some(vec![Part::Text(system_instruction.to_string())]),
        }),
        ..Default::default()
    };

    let result = gemini
        .generate_content(&request, "gemini-1.0-pro-002")
        .await?;

    println!("Response: {:?}", result.candidates[0].get_text().unwrap());

    Ok(())
}
