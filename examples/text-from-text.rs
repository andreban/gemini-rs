use std::sync::Arc;

use gemini_rs::prelude::*;

use gcp_auth::AuthenticationManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let authentication_manager = Arc::new(AuthenticationManager::new().await?);
    let api_endpoint = std::env::var("API_ENDPOINT")?;
    let project_id = std::env::var("PROJECT_ID")?;
    let location_id = std::env::var("LOCATION_ID")?;

    let vertex_client = VertexClient::new(
        authentication_manager,
        api_endpoint,
        project_id,
        location_id,
    );

    let prompt = "What is the airspeed of an unladen swallow?";
    let result = vertex_client.prompt_text(prompt, None).await?;
    println!("Response: {}", result);

    Ok(())
}
