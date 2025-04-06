use std::fmt::Display;

use reqwest_eventsource::CannotCloneRequestError;

use crate::types;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(std::env::VarError),
    HttpClient(reqwest::Error),
    Token(gcp_auth::Error),
    Serde(serde_json::Error),
    VertexError(types::VertexApiError),
    NoCandidatesError,
    CannotCloneRequestError(CannotCloneRequestError),
    EventSourceError(reqwest_eventsource::Error),
    EventSourceClosedError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Env(e) => write!(f, "Environment variable error: {}", e),
            Error::HttpClient(e) => write!(f, "HTTP Client error: {}", e),
            Error::Token(e) => write!(f, "Token error: {}", e),
            Error::Serde(e) => write!(f, "Serde error: {}", e),
            Error::VertexError(e) => {
                write!(f, "Vertex error: {}", e)
            }
            Error::NoCandidatesError => {
                write!(f, "No candidates returned for the prompt")
            }
            Error::CannotCloneRequestError(e) => {
                write!(f, "Cannot clone request: {}", e)
            }
            Error::EventSourceError(e) => {
                write!(f, "EventSourrce Error: {}", e)
            }
            Error::EventSourceClosedError => {
                write!(f, "EventSource closed error")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpClient(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::Env(e)
    }
}

impl From<gcp_auth::Error> for Error {
    fn from(e: gcp_auth::Error) -> Self {
        Error::Token(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<types::VertexApiError> for Error {
    fn from(e: types::VertexApiError) -> Self {
        Error::VertexError(e)
    }
}

impl From<CannotCloneRequestError> for Error {
    fn from(e: CannotCloneRequestError) -> Self {
        Error::CannotCloneRequestError(e)
    }
}

impl From<reqwest_eventsource::Error> for Error {
    fn from(e: reqwest_eventsource::Error) -> Self {
        Error::EventSourceError(e)
    }
}
