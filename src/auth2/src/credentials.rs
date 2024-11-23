use dirs::home_dir;
use std::env::var;
use std::path::{Path, PathBuf};
use tokio::fs::read_to_string;
use std::result::Result::Ok;
use async_trait::async_trait;


mod user_credential;
use user_credential::UserCredential;

#[async_trait]
pub trait Credential: Send + Sync {
    async fn get_token(&mut self) -> Result<crate::token::Token, anyhow::Error>;
    async fn get_authorization_header(&mut self) -> Result<String, anyhow::Error> {
        let token = self.get_token().await?;
        Ok(format!("{} {}", token.token_type, token.token))
    }
    fn get_quota_project_id(&self) -> Result<String, anyhow::Error>;
    fn get_universe_domain(&self) -> Result<String, anyhow::Error>;
}

pub async fn create_access_token_credential() -> Result<Box<dyn Credential>, anyhow::Error> {
    let credential_env = var("GOOGLE_APPLICATION_CREDENTIALS");
    let adc_path = {
        if let Ok(credential_env) = credential_env {
            AdcFilePath::try_from(credential_env)
        } else {
            AdcFilePath::default()
        }
    };
    if let Ok(path) = adc_path {
        let credential = from_adc(&path).await?;
        return Ok(credential);
    }
    
    Err(anyhow::anyhow!(format!(
        "Could not create a credential."
    )))
}

async fn from_adc(path: &AdcFilePath) -> Result<Box<dyn Credential>, anyhow::Error> {
    let data = read_to_string(path).await?;
    if let Ok(user_credential) = UserCredential::from_json(&data) {
        return Ok(Box::new(user_credential))
    }

    Err(anyhow::anyhow!(format!(
        "Could not create a credential from {:?}",
        path
    )))
}

#[derive(Debug)]
pub(crate) struct AdcFilePath(Box<PathBuf>);
impl AdcFilePath {
    pub(crate) fn default() -> Result<Self, anyhow::Error> {
        if let Some(home) = home_dir() {
            let p = home
                .join(".config")
                .join("gcloud")
                .join("application_default_credentials.json");
            if !p.exists() || !p.is_file() {
                return Err(anyhow::anyhow!(
                    "ADC file path does not exist or is not a file."
                ));
            }
            return Ok(AdcFilePath(p.into()));
        }
        Err(anyhow::anyhow!(
            "Could not find an ADC file in the gcloud config directory."
        ))
    }
}
impl TryFrom<String> for AdcFilePath {
    // TODO: Make the error type correct.
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let p = Path::new(&value).to_owned();
        if !p.exists() || !p.is_file() {
            return Err(anyhow::anyhow!(
                "ADC file path does not exist or is not a file."
            ));
        }
        Ok(AdcFilePath(Box::new(p)))
    }
}
impl AsRef<Path> for AdcFilePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

// #[derive(Debug)]
// pub struct Credential {
//     user_account_token_source: UserAccountTokenSource
// }

// impl Credential {
//     pub fn create_access_token_credential() -> Result<Self, anyhow::Error> {
//         match Credential::read_well_known_location() {
//             std::result::Result::Ok(user_account_token_source) => {
//                 Ok(Credential {user_account_token_source})
//             },
//             Err(e) => {
//                 return Err(e);
//             }
//         }
//     }

//     pub fn get_token(&self) -> Result<Token, anyhow::Error> {
//         self.user_account_token_source.get_token()
//     }

//     fn read_well_known_location() -> Result<UserAccountTokenSource, anyhow::Error> {
//         let mut path = PathBuf::from(std::env::var("HOME").unwrap());
//         path.push(".config/gcloud/application_default_credentials.json");
    
//         let file = fs::File::open(path)?;
//         let credentials_file: CredentialsFile = from_reader(file)?;
    
//         // Extract values from CredentialsFile struct
//         let client_id = credentials_file.client_id.unwrap_or_default();
//         let client_secret = credentials_file.client_secret.unwrap_or_default();
//         let refresh_token = credentials_file.refresh_token.unwrap_or_default();
    
//         // These values are typically hardcoded or fetched from a well-known configuration
//         let token_url = "https://oauth2.googleapis.com/token".to_string();
//         let redirect_url = "http://localhost:8080".to_string(); // Example redirect URL
    
//         Ok(UserAccountTokenSource {
//             client_id,
//             client_secret,
//             token_url,
//             redirect_url,
//             refresh_token,
//             token: None
//         })
//     }
// }

// #[derive(Debug)]
// pub struct UserAccountTokenSource {
//     client_id: String,
//     client_secret: String,
//     token_url: String,
//     redirect_url: String,
//     refresh_token: String,

//     token: Option<String>
// }

// impl UserAccountTokenSource {
//     pub fn get_token(&self) -> Result<Token, anyhow::Error> {

//         let data = json!({
//             "client_id": &self.client_id,
//             "client_secret": &self.client_secret,
//             "grant_type": "refresh_token",
//             "refresh_token": &self.refresh_token,
//         });

//         let client = Client::builder().https_only(true). build()?;

//         let response = client.post(self.token_url.to_string()).json(&data).send()?;
//         let status = response.status();
//         let response_text = response.text().unwrap();

//         match status {
//             StatusCode::OK => {
//                 let token: Token = serde_json::from_str(&response_text)?;
//                 Ok(token)
//             }
//             _ => { Err(anyhow!(response_text)) }
            
//         }
//     }
// }

// #[allow(dead_code)]
// #[derive(Deserialize, Serialize)]
// #[cfg_attr(test, derive(Debug))]
// pub struct CredentialsFile {
//     // Service Account fields
//     pub client_email: Option<String>,
//     pub private_key_id: Option<String>,
//     pub private_key: Option<String>,
//     pub auth_uri: Option<String>,
//     pub token_uri: Option<String>,
//     pub project_id: Option<String>,

//     // User Credential fields
//     // (These typically come from gcloud auth.)
//     pub client_secret: Option<String>,
//     pub client_id: Option<String>,
//     pub refresh_token: Option<String>,
// }

// #[derive(Clone, Deserialize, Debug)]
// pub struct Token {
//     pub access_token: String,
//     pub token_type: String,
//     pub expires_in: Option<i64>,
// }