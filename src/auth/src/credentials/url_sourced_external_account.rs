use async_trait::async_trait;
use gax::error::CredentialsError;
use serde::{Deserialize, Serialize};
use crate::credentials::{Credentials, Result};
use crate::token::{Token, TokenProvider};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use super::dynamic::CredentialsProvider;
use super::internal::sts::StsClient;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CredentialSourceFormat {
    #[serde(rename = "type")]
    format_type: String,
    subject_token_field_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CredentialSourceHeaders {
    #[serde(flatten)]
    headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CredentialSource {
    url: String,
    headers: Option<CredentialSourceHeaders>,
    format: Option<CredentialSourceFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExternalAccountConfig {
    #[serde(rename = "type")]
    config_type: String,
    audience: String,
    subject_token_type: String,
    token_url: String,
    service_account_impersonation_url: Option<String>,
    credential_source: CredentialSource,
}

#[derive(Debug)]
pub(crate) struct UrlSourcedCredentials<T>
where
    T: TokenProvider,
{
    token_provider: T,
    quota_project_id: Option<String>,
}

pub fn new(external_account_config: Value) -> Result<Credentials> {
    let config: ExternalAccountConfig = serde_json::from_value(external_account_config).unwrap();
    let token_provider = UrlSourcedTokenProvider::new(config).unwrap();
    let credentials = UrlSourcedCredentials {
        token_provider,
        quota_project_id: None,
    };

    Ok(
        Credentials {
            inner: Arc::new(credentials)
        }
    )
}

#[async_trait::async_trait]
impl<T> CredentialsProvider for UrlSourcedCredentials<T>
where
    T: TokenProvider,
{
    async fn token(&self) -> Result<Token> {
        self.token_provider.token().await
    }

    async fn headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>> {
        todo!()
    }
}

#[derive(Debug)]
struct UrlSourcedTokenProvider {
    config: ExternalAccountConfig,
    http_client: Client,
    sts_client: StsClient, // Add StsClient
}

impl UrlSourcedTokenProvider {
    pub fn new(config: ExternalAccountConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        let sts_client = StsClient::new(config.token_url.clone())?; // Initialize StsClient
        Ok(Self { config, http_client, sts_client })
    }
}

#[async_trait]
impl TokenProvider for UrlSourcedTokenProvider {
    async fn token(&self) -> Result<Token> {
        let subject_token = self.fetch_subject_token().await?;
        let token = self.sts_client.request_token(
            self.config.audience.clone(),
            subject_token,
            self.config.subject_token_type.clone(),
        ).await?;
        Ok(token)
    }
}

impl UrlSourcedTokenProvider {
    async fn fetch_subject_token(&self) -> Result<String> {
        let mut request = self.http_client.get(&self.config.credential_source.url);

        if let Some(headers) = &self.config.credential_source.headers {
            for (key, value) in &headers.headers {
                request = request.header(key.as_str(), value.as_str());
            }
        }

        let response = request.send().await.unwrap();

        if !response.status().is_success() {
            // Properly handle error
            return Err(CredentialsError::from_str(false, "message"))
        }

        let response_text = response.text().await.unwrap();

        match &self.config.credential_source.format {
            Some(format) => {
                let json_response: Value = serde_json::from_str(&response_text).unwrap();
                let subject_token = json_response
                    .get(&format.subject_token_field_name)
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap();
                Ok(subject_token)
            }
            None => Ok(response_text),
        }
    }
}