//! Configuration loading and validation.

use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::Path;
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

pub(crate) fn config_file_path() -> Option<PathBuf> {
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
    let path = config_file_path().ok_or(SerisError::InvalidConfig {
        field: "config path",
        reason: "could not be determined",
    })?;
    let required = env::var("SERIS_CONFIG_FILE").is_ok();

    load_config_from_path_with_required(path, required)
}

/// Loads configuration from a specific file path.
pub fn load_config_from_path(path: impl AsRef<Path>) -> Result<AppConfig, Error> {
    load_config_from_path_with_required(path, true)
}

fn load_config_from_path_with_required(
    path: impl AsRef<Path>,
    required: bool,
) -> Result<AppConfig, Error> {
    let path = path.as_ref();

    let c = Config::builder()
        .add_source(File::new(path.to_string_lossy().as_ref(), FileFormat::Toml).required(required))
        .add_source(
            Environment::with_prefix("SERIS")
                .prefix_separator("_")
                .ignore_empty(true),
        )
        .build()?;

    let config: AppConfig = c.try_deserialize()?;
    config.validate()?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::{config_file_path, load_config_from_path, AppConfig};
    use std::env;
    use std::sync::{Mutex, OnceLock};
    use tempfile::tempdir;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn validate_accepts_non_empty_values() {
        let config = AppConfig {
            discord_token: "token".to_string(),
            nasa_api_key: "key".to_string(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn validate_rejects_empty_discord_token() {
        let config = AppConfig {
            discord_token: String::new(),
            nasa_api_key: "key".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn load_config_uses_env_fallbacks() {
        let dir = tempdir().expect("temp dir");
        let config_path = dir.path().join("config.toml");
        std::fs::write(&config_path, "").expect("write empty config");

        let _guard = env_lock().lock().expect("env lock");
        env::set_var("SERIS_DISCORD_TOKEN", "env-token");
        env::set_var("SERIS_NASA_API_KEY", "env-key");

        let config = load_config_from_path(&config_path).expect("load config from env");

        assert_eq!(config.discord_token, "env-token");
        assert_eq!(config.nasa_api_key, "env-key");

        env::remove_var("SERIS_DISCORD_TOKEN");
        env::remove_var("SERIS_NASA_API_KEY");
    }

    #[test]
    fn config_file_path_prefers_explicit_override() {
        let dir = tempdir().expect("temp dir");
        let config_path = dir.path().join("custom.toml");

        let _guard = env_lock().lock().expect("env lock");
        env::set_var("SERIS_CONFIG_FILE", &config_path);

        assert_eq!(config_file_path(), Some(config_path.clone()));

        env::remove_var("SERIS_CONFIG_FILE");
    }
}
