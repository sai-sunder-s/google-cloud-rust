use serde::Deserialize;
use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;


#[derive(Clone, Deserialize, Debug)]
pub struct Token {
    pub token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub metadata: Option<HashMap<String, String>>
}

#[async_trait]
pub(crate) trait TokenProvider: Send + Sync {
    async fn get_token_internal(&self) -> Result<Token>;
}

pub(crate) struct TokenCache<T: TokenProvider> {
    token_provider: T,
    cached_token: Option<Token>,
}

impl<T:TokenProvider> TokenCache<T> {
    pub fn new(token_provider: T) -> Self {
        TokenCache {
            token_provider,
            cached_token: None,
        }
    }

    pub async fn token_non_blocking(&mut self) -> Result<Token> {
        // TODO: Implement real caching mechanism
        if self.cached_token.is_none() {
            self.cached_token = Some(self.token_provider.get_token_internal().await?);
        }

        Ok(self.cached_token.clone().unwrap())
    }
}