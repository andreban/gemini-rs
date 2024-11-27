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

    let system_instruction = Content::builder()
        .add_text_part("Answer as if you were Yoda")
        .build();

    let user_prompt = vec![Content::builder()
        .role(Role::User)
        .add_text_part("What is the airspeed of an unladen swallow?")
        .build()];

    let request = GenerateContentRequest::builder()
        .contents(user_prompt)
        .system_instruction(system_instruction)
        .build();

    let result = gemini
        .generate_content(&request, "gemini-1.0-pro-002")
        .await?;

    println!("Response: {:?}", result.candidates[0].get_text().unwrap());

    Ok(())
}
