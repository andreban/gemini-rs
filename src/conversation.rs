use std::str::FromStr;

use serde::{Deserialize, Serialize};

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation { messages: vec![] }
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
