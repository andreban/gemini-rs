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

    let prompt = "Tell me the story of the genesis of the universe as a bedtime story.";
    let request = GenerateContentRequest::builder()
        .add_text_content(Role::User, prompt)
        .build();

    let queue = gemini.stream_generate_content(&request, "gemini-pro").await;

    while let Some(response) = queue.pop().await {
        match response {
            Ok(result) => {
                let text = result
                    .candidates
                    .iter()
                    .filter_map(|c| c.get_text())
                    .collect::<String>();
                print!("{}", text);
            }
            Err(error) => {
                println!("{error}");
            }
        }
    }

    Ok(())
}
