use chromadb::v1::{client::ChromaClientOptions, ChromaClient};
use teloxide::prelude::*;

use crate::agent::Agent;
use crate::config::Config;
use crate::database::{Database, DatabaseSetupError};

pub struct State {
    sqlite_database: Database,
    chroma_database: ChromaClient,
    agent: Agent,
}

#[allow(dead_code)]
impl State {
    pub fn sqlite_database(&self) -> &Database {
        &self.sqlite_database
    }

    pub fn chroma_database(&self) -> &ChromaClient {
        &self.chroma_database
    }

    pub fn agent(&self) -> &Agent {
        &self.agent
    }

    pub async fn from_config(config: &Config) -> Result<(Self, Bot), StateSetupError> {
        let telegram_bot = Bot::new(config.telegram_bot_token().to_string());

        let sqlite_database = Database::connect(config.sqlite_database_url()).await?;

        let chroma_database = ChromaClient::new(ChromaClientOptions {
            url: config.chroma_database_url().to_string(),
        });

        let agent = Agent::new(config.ollama_server_url());

        Ok((
            Self {
                sqlite_database,
                chroma_database,
                agent,
            },
            telegram_bot,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateSetupError {
    #[error("failed to setup the database: {0}")]
    DatabaseSetupError(#[from] DatabaseSetupError),
}
