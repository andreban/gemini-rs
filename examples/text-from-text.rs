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
    let request = GenerateContentRequest::builder()
        .add_content(
            Content::builder()
                .role(Role::User)
                .add_part(Part::Text(prompt.to_string()))
                .build(),
        )
        .build();
    let response = gemini.generate_content(&request, "gemini-pro").await?;
    println!("Response: {:?}", response.candidates[0].get_text().unwrap());

    Ok(())
}
