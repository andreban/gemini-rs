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

    let prompt = "What day is today?";

    let request = GenerateContentRequest {
        contents: vec![Content {
            role: Some("user".to_string()),
            parts: Some(vec![Part::Text(prompt.to_string())]),
        }],
        tools: Some(vec![Tools {
            google_search_retrieval: Some(GoogleSearchRetrieval::default()),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let result = gemini
        .generate_content(&request, "gemini-1.0-pro-002")
        .await?;

    println!("Response: {:?}", result.candidates[0].get_text().unwrap());

    Ok(())
}
