use std::{error::Error, io::Cursor};

use gemini_rs::prelude::{
    GeminiClient, PersonGeneration, PredictImageRequest, PredictImageRequestParameters,
    PredictImageRequestParametersOutputOptions, PredictImageRequestPrompt,
    PredictImageSafetySetting,
};
use image::{ImageFormat, ImageReader};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
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

    let prompt = "
    Create an image of a tuxedo cat riding a rocket to the moon.";
    let request = PredictImageRequest {
        instances: vec![PredictImageRequestPrompt {
            prompt: prompt.to_string(),
        }],
        parameters: PredictImageRequestParameters {
            sample_count: 1,
            aspect_ratio: Some("1:1".to_string()),
            output_options: Some(PredictImageRequestParametersOutputOptions {
                mime_type: Some("image/jpeg".to_string()),
                compression_quality: Some(75),
            }),
            person_generation: Some(PersonGeneration::AllowAll),
            safety_setting: Some(PredictImageSafetySetting::BlockOnlyHigh),
            ..Default::default()
        },
    };

    println!("Request: {:#?}", serde_json::to_string(&request).unwrap());

    let mut result = gemini
        .predict_image(&request, "imagen-3.0-fast-generate-001")
        .await?;

    let result = result.predictions.pop().unwrap();

    let format = ImageFormat::from_mime_type(result.mime_type).unwrap();
    let img =
        ImageReader::with_format(Cursor::new(result.bytes_base64_encoded), format).decode()?;
    img.save("output.jpg")?;
    Ok(())
}
