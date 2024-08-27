use config::{Config as ConfigTrait, ConfigError};
pub use deadpool_postgres::{Client, Pool};
use deadpool_postgres::{Config as PgConfig, Runtime};
use dotenvy::dotenv;
use serde::Deserialize;
pub use tokio_postgres::{Error, NoTls};
use tokio_postgres::config::SslMode;
use tokio_postgres_rustls::{MakeRustlsConnect, RustlsConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: PgConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        ConfigTrait::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

pub fn create_pool() -> Pool {
    dotenv().ok();
    let cfg = Config::from_env().unwrap();

    // cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
     // Create a RustlsConfig
     let rustls_config = RustlsConfig::default();
     let tls = MakeRustlsConnect::new(rustls_config);
 
     // Update the pool creation to use SSL
     cfg.pg.create_pool(Some(Runtime::Tokio1), tls).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_pg_pool() {
        // Load .env file
        dotenv().ok();

        // Ensure PG__URL is set in the .env file
        env::var("PG__URL").expect("PG__URL must be set in .env file for tests");

        // Create the pool
        let pool = create_pool();

        // Try to get a connection from the pool
        let client = pool.get().await.expect("Failed to get client from pool");
        let result = client
            .query_one("SELECT 1", &[])
            .await
            .expect("Query failed");
        assert_eq!(result.get::<_, i32>(0), 1);
    }
}
