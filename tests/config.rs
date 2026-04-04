use std::fs;

use seris::config::load_config_from_path;
use tempfile::tempdir;

#[test]
fn loads_config_file() {
    let dir = tempdir().expect("temp dir");
    let config_path = dir.path().join("config.toml");

    fs::write(
        &config_path,
        "discord_token = \"token\"\nnasa_api_key = \"key\"\n",
    )
    .expect("write config");

    let config = load_config_from_path(&config_path).expect("load config");

    assert_eq!(config.discord_token, "token");
    assert_eq!(config.nasa_api_key, "key");
}

#[test]
fn rejects_invalid_config_values() {
    let dir = tempdir().expect("temp dir");
    let config_path = dir.path().join("config.toml");

    fs::write(
        &config_path,
        "discord_token = \"\"\nnasa_api_key = \"key\"\n",
    )
    .expect("write config");

    assert!(load_config_from_path(&config_path).is_err());
}
