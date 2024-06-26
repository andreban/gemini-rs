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

    let embedding_request = TextEmbeddingRequest {
        instances: vec![
            TextEmbeddingRequestInstance {
                title: Some(String::from("Embed testing")),
                content: String::from("Embed testing"),
                task_type: String::from("RETRIEVAL_DOCUMENT"),
            },
            TextEmbeddingRequestInstance {
                title: Some(String::from("Embed testing 2")),
                content: String::from("Embed testing 2"),
                task_type: String::from("RETRIEVAL_DOCUMENT"),
            },
        ],
    };

    let result = gemini
        .text_embeddings(&embedding_request, "textembedding-gecko@003")
        .await?
        .into_result()?;
    println!("Response: {:?}", result);

    Ok(())
}
