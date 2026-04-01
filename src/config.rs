use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub discord_token: String,
    pub nasa_api_key: String,
}

fn default_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("SERIS_CONFIG_FILE") {
        return Some(PathBuf::from(path));
    }

    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg_config_home).join("seris").join("config.toml"));
    }

    env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".config").join("seris").join("config.toml"))
}

pub fn load_config() -> AppConfig {
    let path = default_config_path().expect("cannot determine config file path");
    let required = env::var("SERIS_CONFIG_FILE").is_ok();

    let c = Config::builder()
        .add_source(File::new(path.to_string_lossy().as_ref(), FileFormat::Toml).required(required))
        .build()
        .expect("cannot build config");

    c.try_deserialize()
        .expect("cannot deserialize config from TOML file")
}
