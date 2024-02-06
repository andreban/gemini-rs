use std::{sync::Arc, time::Duration};

use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use gemini_rs::prelude::*;

use gcp_auth::AuthenticationManager;
use indicatif::{ProgressBar, ProgressStyle};

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

    let mut conversation = Conversation::new();
    loop {
        let message: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("user")
            .interact_text()?;

        // Exit the conversation if the user types "exit"
        if message == "exit" {
            break;
        }

        // Push the user's message to the conversation.
        conversation.push_message(Message::new(Role::User, &message));

        // Show a spinner while the model is thinking.
        let progress = ProgressBar::new_spinner();
        progress.enable_steady_tick(Duration::from_millis(120));
        progress.set_style(ProgressStyle::with_template("{spinner:.green} {msg}")?);
        progress.set_message("Thinking...");

        // Prompt the model with the conversation so far.
        let response = vertex_client.prompt_conversation(&conversation).await?;

        // Stop the spinner and clear the terminal.
        progress.finish_and_clear();

        // Print the model's response.
        println!(
            "✨ {} {} {}",
            style(response.role.to_string()).bold(),
            style("·").dim(),
            style(&response.text).cyan()
        );
        conversation.push_message(response);
    }

    Ok(())
}
