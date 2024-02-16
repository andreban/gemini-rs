use std::sync::Arc;

use gemini_rs::prelude::*;

use gcp_auth::AuthenticationManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let authentication_manager = Arc::new(AuthenticationManager::new().await?);
    let api_endpoint = std::env::var("API_ENDPOINT")?;
    let project_id = std::env::var("PROJECT_ID")?;
    let location_id = std::env::var("LOCATION_ID")?;

    let gemini = GeminiClient::new(
        authentication_manager,
        api_endpoint,
        project_id,
        location_id,
    );

    let prompt = "Tell me the story of the genesis of the universe as a bedtime story.";
    let request = GenerateContentRequest::from_prompt(prompt, None);
    let queue = gemini
        .streaming_stream_generate_content(&request, Model::GeminiPro)
        .await;

    while let Some(chunk) = queue.pop().await {
        if let ResponseStreamChunk::Ok(ok_response) = chunk {
            let text = ok_response
                .candidates
                .iter()
                .filter_map(|c| c.get_text())
                .collect::<String>();
            print!("{}", text);
        }
    }

    Ok(())
}
