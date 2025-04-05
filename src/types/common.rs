use std::{collections::HashMap, fmt::Display, str::FromStr, vec};

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

    pub fn builder() -> ContentBuilder {
        ContentBuilder::default()
    }
}

#[derive(Default)]
pub struct ContentBuilder {
    content: Content,
}

impl ContentBuilder {
    pub fn add_text_part<T: Into<String>>(self, text: T) -> Self {
        self.add_part(Part::Text(text.into()))
    }

    pub fn add_part(mut self, part: Part) -> Self {
        match &mut self.content.parts {
            Some(parts) => parts.push(part),
            None => self.content.parts = Some(vec![part]),
        }
        self
    }

    pub fn role(mut self, role: Role) -> Self {
        self.content.role = Some(role);
        self
    }

    pub fn build(self) -> Content {
        self.content
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::User => "user",
            Role::Model => "model",
        };
        f.write_str(role_str)
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
