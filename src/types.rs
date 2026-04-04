use crate::config::AppConfig;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerisError {
    #[error(transparent)]
    Config(#[from] ::config::ConfigError),

    #[error(transparent)]
    Http(#[from] ::reqwest::Error),

    #[error(transparent)]
    Serenity(#[from] ::serenity::Error),

    #[error("invalid configuration for {field}: {reason}")]
    InvalidConfig {
        field: &'static str,
        reason: &'static str,
    },
}

pub struct Data {
    pub config: AppConfig,
}

pub type Error = SerisError;
pub type Context<'a> = poise::Context<'a, Data, Error>;
