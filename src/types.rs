//! Shared application state and error types.

use crate::config::AppConfig;
use reqwest::StatusCode;
use std::sync::Arc;
use std::time::Instant;
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

    /// Errors while interacting with SQLite persistence.
    #[error(transparent)]
    Sqlite(#[from] ::rusqlite::Error),

    /// Errors from the SQLite connection pool.
    #[error(transparent)]
    Pool(#[from] ::r2d2::Error),

    /// I/O errors from local runtime services.
    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    /// HTTP request timed out.
    #[error("request to {service} timed out")]
    Timeout {
        /// Service that timed out.
        service: &'static str,
    },

    /// HTTP response returned a non-retryable status.
    #[error("request to {service} failed with status {status}")]
    HttpStatus {
        /// Service that returned the status.
        service: &'static str,
        /// Returned HTTP status.
        status: StatusCode,
    },

    /// Circuit breaker is open for this service.
    #[error("service {service} is temporarily unavailable")]
    CircuitOpen {
        /// Service currently blocked by the circuit breaker.
        service: &'static str,
    },

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
    /// SQLite-backed persistence.
    pub database: Arc<crate::database::Database>,
    /// When the process started.
    pub started_at: Instant,
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
