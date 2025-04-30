use async_trait::async_trait;
use gax::error::CredentialsError;
use crate::token::{Token, TokenProvider};
use crate::credentials::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Deserialize)]
struct StsTokenResponse {
    access_token: String,
    expires_in: Option<u64>,
}

#[derive(Debug)]
pub struct StsClient {
    client: Client,
    token_url: String,
}

impl StsClient {
    pub fn new(token_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        Ok(Self { client, token_url })
    }

    pub async fn request_token(
        &self,
        audience: String,
        subject_token: String,
        subject_token_type: String,
    ) -> Result<Token> {
        let mut params = HashMap::new();
        params.insert("grant_type", "urn:ietf:params:oauth:grant-type:token-exchange");
        params.insert("audience", &audience);
        params.insert("subject_token", &subject_token);
        params.insert("subject_token_type", &subject_token_type);
        params.insert("requested_token_type", "urn:ietf:params:oauth:token-type:access_token");

        let response = self
            .client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            // Properly handle error
            return Err(CredentialsError::from_str(false, "message"))
        }

        let sts_response: StsTokenResponse = response.json().await.unwrap();

        Ok(Token {
            token: sts_response.access_token,
            token_type: "Bearer".to_string(),
            expires_at: sts_response
                .expires_in
                .map(|d| std::time::Instant::now() + Duration::from_secs(d)),
            metadata: None,
        })
    }
}