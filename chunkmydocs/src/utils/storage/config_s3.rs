use aws_credential_types::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::{Client, Config as S3Config};
use config::{Config as ConfigTrait, ConfigError};
use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::Once;

static INIT: Once = Once::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    access_key: String,
    secret_key: String,
    endpoint: Option<String>,
    region: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        INIT.call_once(|| {
            dotenv().ok();
        });

        ConfigTrait::builder()
            .add_source(config::Environment::default().prefix("AWS").separator("__"))
            .build()?
            .try_deserialize()
    }
}

pub async fn create_client() -> Result<Client, ConfigError> {
    let config = Config::from_env()?;
    let creds = Credentials::from_keys(config.access_key, config.secret_key, None);
    let config_region = config.region.unwrap_or_else(|| "us-west-1".to_string());
    let aws_config = if let Some(endpoint) = config.endpoint {
        S3Config::builder()
            .credentials_provider(creds)
            .region(Region::new(config_region))
            .endpoint_url(endpoint)
            .build()
    } else {
        S3Config::builder()
            .credentials_provider(creds)
            .region(Region::new(config_region))
            .build()
    };
    let client = aws_sdk_s3::Client::from_conf(aws_config);
    Ok(client)
}