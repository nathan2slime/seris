//! Configuration loading and validation.

use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

use crate::types::{Error, SerisError};

/// Application settings loaded from the Seris config file.
#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    /// Discord bot token used to authenticate the client.
    pub discord_token: String,
    /// NASA API key used for APOD requests.
    pub nasa_api_key: String,
}

impl AppConfig {
    /// Validates required configuration values.
    pub fn validate(&self) -> Result<(), Error> {
        if self.discord_token.trim().is_empty() {
            return Err(SerisError::InvalidConfig {
                field: "discord_token",
                reason: "must not be empty",
            });
        }

        if self.nasa_api_key.trim().is_empty() {
            return Err(SerisError::InvalidConfig {
                field: "nasa_api_key",
                reason: "must not be empty",
            });
        }

        Ok(())
    }
}

fn default_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("SERIS_CONFIG_FILE") {
        return Some(PathBuf::from(path));
    }

    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        return Some(
            PathBuf::from(xdg_config_home)
                .join("seris")
                .join("config.toml"),
        );
    }

    env::var("HOME").ok().map(|home| {
        PathBuf::from(home)
            .join(".config")
            .join("seris")
            .join("config.toml")
    })
}

/// Loads and validates the application configuration.
pub fn load_config() -> Result<AppConfig, Error> {
    let path = default_config_path().ok_or(SerisError::InvalidConfig {
        field: "config path",
        reason: "could not be determined",
    })?;
    let required = env::var("SERIS_CONFIG_FILE").is_ok();

    let c = Config::builder()
        .add_source(File::new(path.to_string_lossy().as_ref(), FileFormat::Toml).required(required))
        .build()?;

    let config: AppConfig = c.try_deserialize()?;
    config.validate()?;
    Ok(config)
}
