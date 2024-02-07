use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{client::GeminiClient, error::Result, prelude::TokenProvider};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    User,
    Model,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::User => "user".to_string(),
            Role::Model => "model".to_string(),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "model" => Ok(Role::Model),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub text: String,
}

impl Message {
    pub fn new(role: Role, text: &str) -> Self {
        Message {
            role,
            text: text.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dialogue {
    messages: Vec<Message>,
}

impl Dialogue {
    pub fn new() -> Self {
        Dialogue { messages: vec![] }
    }

    pub async fn do_turn<T: TokenProvider + Clone>(
        &mut self,
        gemini: &GeminiClient<T>,
        message: &str,
    ) -> Result<Message> {
        self.messages.push(Message::new(Role::User, message));
        let response = gemini.prompt_conversation(&self.messages).await?;
        self.messages.push(response.clone());
        Ok(response)
    }
}
