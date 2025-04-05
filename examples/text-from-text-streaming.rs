use gemini_rs::prelude::*;
use tokio_stream::StreamExt;

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

    let prompt = vec![Content::builder()
        .role(Role::User)
        .add_text_part("Tell me the story of the genesis of the universe as a bedtime story.")
        .build()];

    let request = GenerateContentRequest::builder().contents(prompt).build();

    let mut queue = gemini
        .generate_content_stream(&request, "gemini-2.0-flash-001")
        .await?;

    while let Some(Ok(response)) = queue.next().await {
        println!("Response: {:?}", response);
    }

    Ok(())
}
