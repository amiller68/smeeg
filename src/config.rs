use dotenvy::dotenv;
use std::env;

use url::Url;

#[derive(Debug)]
pub struct Config {
    // TODO: this should be a secret
    telegram_bot_token: String,

    chroma_database_url: Url,
    sqlite_database_url: Url,
    ollama_server_url: Url,
}

// TODO: arg parsing
impl Config {
    pub fn parse_env() -> Result<Config, ConfigError> {
        if dotenv().is_err() {
            tracing::warn!("No .env file found");
        }

        let chroma_database_url_str = match env::var("CHROMA_DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                tracing::warn!("No CHROMA_DATABASE_URL found in .env, using default");
                "http://localhost:8000".to_string()
            }
        };
        let chroma_database_url = Url::parse(&chroma_database_url_str)?;

        let sqlite_database_url_str = match env::var("SQLITE_DATABASE_URL") {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("No SQLITE_DATABASE_URL found in .env");
                return Err(ConfigError::InvalidEnv(e));
            }
        };
        let sqlite_database_url = Url::parse(&sqlite_database_url_str)?;

        let telegram_bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
            Ok(token) => token,
            Err(e) => {
                tracing::warn!("No TELEGRAM_BOT_TOKEN found in .env");
                return Err(ConfigError::InvalidEnv(e));
            }
        };

        let ollama_server_url_str = match env::var("OLLAMA_SERVER_URL") {
            Ok(url) => url,
            Err(_) => {
                tracing::warn!("No OLLAMA_SERVER_URL found in .env, using default");
                "http://localhost:11434".to_string()
            }
        };
        let ollama_server_url = Url::parse(&ollama_server_url_str)?;

        Ok(Config {
            chroma_database_url,
            sqlite_database_url,
            telegram_bot_token,
            ollama_server_url,
        })
    }

    pub fn telegram_bot_token(&self) -> &str {
        &self.telegram_bot_token
    }

    pub fn chroma_database_url(&self) -> &Url {
        &self.chroma_database_url
    }

    pub fn sqlite_database_url(&self) -> &Url {
        &self.sqlite_database_url
    }

    pub fn ollama_server_url(&self) -> &Url {
        &self.ollama_server_url
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Missing Env: {0}")]
    InvalidEnv(#[from] env::VarError),
}
