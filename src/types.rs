//! Shared application state and error types.

use crate::config::AppConfig;
use thiserror::Error;

/// Application-wide error type.
#[derive(Debug, Error)]
pub enum SerisError {
    /// Errors while reading or parsing configuration.
    #[error(transparent)]
    Config(#[from] ::config::ConfigError),

    /// Errors while performing HTTP requests.
    #[error(transparent)]
    Http(#[from] ::reqwest::Error),

    /// Errors returned by Serenity.
    #[error(transparent)]
    Serenity(Box<::serenity::Error>),

    /// Configuration values that failed validation.
    #[error("invalid configuration for {field}: {reason}")]
    InvalidConfig {
        /// Name of the invalid field.
        field: &'static str,
        /// Why the value was rejected.
        reason: &'static str,
    },
}

/// Shared data made available to command handlers.
pub struct Data {
    /// Loaded application configuration.
    pub config: AppConfig,
}

/// Canonical error alias used by command handlers.
pub type Error = SerisError;
/// Convenience alias for a Poise command context.
pub type Context<'a> = poise::Context<'a, Data, Error>;

impl From<::serenity::Error> for SerisError {
    fn from(err: ::serenity::Error) -> Self {
        Self::Serenity(Box::new(err))
    }
}
