use log::info;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::{path::PathBuf, process};

use crate::{
    services::http,
    types::{Error, SerisError},
};

const GITHUB_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/nathan2slime/seris/releases/latest";
const GITHUB_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

/// Runs a self-update against the latest GitHub release.
pub async fn run_self_update() -> Result<(), Error> {
    if !cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        return Err(update_error(
            "self-update is only available for Linux x86_64 builds",
        ));
    }

    let release = latest_release(GITHUB_LATEST_RELEASE_URL).await?;
    let current_version = env!("CARGO_PKG_VERSION");
    let latest_version = release.tag_name.trim_start_matches('v');

    if latest_version == current_version {
        info!("Seris is already up to date ({})", release.tag_name);
        return Ok(());
    }

    let asset_name = format!("seris-{}-{}", release.tag_name, TARGET_TRIPLE);
    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name == asset_name)
        .ok_or_else(|| {
            update_error(format!(
                "release {} does not include asset {asset_name}",
                release.tag_name
            ))
        })?;

    let current_exe = std::env::current_exe()?;
    let parent = current_exe
        .parent()
        .ok_or_else(|| update_error("unable to locate the current executable directory"))?;
    let tmp_path = temp_update_path(parent, &release.tag_name);

    let bytes = download_asset(&asset.browser_download_url).await?;
    if bytes.is_empty() {
        return Err(update_error("downloaded update payload was empty"));
    }

    std::fs::write(&tmp_path, &bytes)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&tmp_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&tmp_path, perms)?;
    }

    std::fs::rename(&tmp_path, &current_exe)?;
    info!("updated Seris to {}", release.tag_name);
    Ok(())
}

async fn latest_release(url: &str) -> Result<GitHubRelease, Error> {
    let url = url.to_string();

    http::get_json("github-release", move || {
        http::client()
            .get(&url)
            .header(USER_AGENT, GITHUB_USER_AGENT)
            .header(ACCEPT, "application/vnd.github+json")
    })
    .await
}

async fn download_asset(url: &str) -> Result<Vec<u8>, Error> {
    let response = http::client()
        .get(url)
        .header(USER_AGENT, GITHUB_USER_AGENT)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(update_error(format!(
            "download failed with status {}",
            response.status()
        )));
    }

    Ok(response.bytes().await?.to_vec())
}

fn temp_update_path(parent: &std::path::Path, tag_name: &str) -> PathBuf {
    parent.join(format!(".seris-update-{}-{tag_name}", process::id()))
}

fn update_error(message: impl Into<String>) -> Error {
    SerisError::Update {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{latest_release, temp_update_path};
    use crate::test_utils::spawn_json_server;

    #[tokio::test]
    async fn parses_release_metadata() {
        let server = spawn_json_server(
            r#"{"tag_name":"v9.9.9","assets":[{"name":"seris-v9.9.9-x86_64-unknown-linux-gnu","browser_download_url":"https://example.com/seris"}]}"#,
        )
        .await;

        let release = latest_release(&server.url).await.expect("release metadata");

        assert_eq!(release.tag_name, "v9.9.9");
        assert_eq!(
            release.assets[0].name,
            "seris-v9.9.9-x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn build_temp_update_path_uses_process_id() {
        let path = temp_update_path(std::path::Path::new("/tmp"), "v1.2.3");

        assert!(path.to_string_lossy().contains(".seris-update-"));
        assert!(path.to_string_lossy().contains("v1.2.3"));
    }
}
