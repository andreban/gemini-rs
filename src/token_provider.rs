use std::sync::Arc;

use gcp_auth::AuthenticationManager;

use crate::error::Result;

pub trait TokenProvider {
    fn get_token(&self, scope: &[&str])
        -> impl std::future::Future<Output = Result<String>> + Send;
}

impl TokenProvider for Arc<AuthenticationManager> {
    async fn get_token(&self, scope: &[&str]) -> Result<String> {
        match AuthenticationManager::get_token(self, scope).await {
            Ok(token) => Ok(token.as_str().to_string()),
            Err(e) => Err(e.into()),
        }
    }
}

impl TokenProvider for AuthenticationManager {
    async fn get_token(&self, scope: &[&str]) -> Result<String> {
        match AuthenticationManager::get_token(self, scope).await {
            Ok(token) => Ok(token.as_str().to_string()),
            Err(e) => Err(e.into()),
        }
    }
}
