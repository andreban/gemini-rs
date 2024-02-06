use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(std::env::VarError),
    HttpClient(reqwest::Error),
    Token(gcp_auth::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Env(e) => write!(f, "Environment variable error: {}", e),
            Error::HttpClient(e) => write!(f, "HTTP Client error: {}", e),
            Error::Token(e) => write!(f, "Token error: {}", e),
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
