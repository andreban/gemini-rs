use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: Option<Role>,
    pub parts: Option<Vec<Part>>,
}

impl Content {
    pub fn get_text(&self) -> Option<String> {
        self.parts.as_ref().map(|parts| {
            parts
                .iter()
                .filter_map(|part| match part {
                    Part::Text(text) => Some(text.clone()),
                    _ => None,
                })
                .collect::<String>()
        })
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "camelCase")]
pub enum Part {
    Text(String),
    InlineData {
        mime_type: String,
        data: String,
    },
    FileData {
        mime_type: String,
        file_uri: String,
    },
    FunctionCall {
        name: String,
        args: HashMap<String, String>,
    },
}
