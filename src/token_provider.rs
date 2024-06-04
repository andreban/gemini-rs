use std::sync::Arc;

use crate::error::Result;

pub trait TokenProvider {
    fn get_token(&self, scope: &[&str])
        -> impl std::future::Future<Output = Result<String>> + Send;
}

impl<'a> TokenProvider for Arc<dyn gcp_auth::TokenProvider + 'a> {
    async fn get_token(&self, scope: &[&str]) -> Result<String> {
        let token = self.token(scope).await;
        match token {
            Ok(token) => Ok(token.as_str().to_string()),
            Err(e) => Err(e.into()),
        }
    }
}
