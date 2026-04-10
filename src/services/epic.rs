//! NASA EPIC API helpers and response models.

use std::sync::OnceLock;

use reqwest::Url;
use serde::Deserialize;

use super::http;
use crate::types::Error;

const API_URL: &str = "https://api.nasa.gov/EPIC";
const SERVICE_NAME: &str = "nasa-epic";

fn api_url() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();
    URL.get_or_init(|| Url::parse(API_URL).expect("valid EPIC url"))
}

/// EPIC image metadata returned by NASA.
#[derive(Clone, Deserialize, Debug)]
struct EpicImageMetadata {
    /// Image identifier.
    pub image: String,
    /// Image caption.
    pub caption: String,
    /// Timestamp string returned by the API.
    pub date: String,
}

/// Resolved EPIC image used by the bot.
#[derive(Clone, Debug)]
pub struct EpicImage {
    /// Image identifier.
    pub image: String,
    /// Image caption.
    pub caption: String,
    /// Timestamp string returned by the API.
    pub date: String,
    /// Resolved archive image URL.
    pub url: String,
}

impl EpicImage {
    /// Returns the date portion used for display.
    pub fn short_date(&self) -> &str {
        self.date.split_whitespace().next().unwrap_or(&self.date)
    }
}

fn archive_url(kind: &str, date: &str, image: &str, api_key: &str) -> String {
    let day = date.split_whitespace().next().unwrap_or(date);
    let mut parts = day.split('-');
    let year = parts.next().unwrap_or("0000");
    let month = parts.next().unwrap_or("00");
    let day = parts.next().unwrap_or("00");

    format!(
        "https://api.nasa.gov/EPIC/archive/{kind}/{year}/{month}/{day}/png/{image}.png?api_key={api_key}"
    )
}

fn endpoint(kind: &str, date: Option<&str>) -> String {
    let base = api_url().as_str().trim_end_matches('/');

    match date {
        Some(date) => format!("{base}/api/{kind}/date/{date}"),
        None => format!("{base}/api/{kind}/images"),
    }
}

/// Fetches EPIC images from a specific base URL.
pub async fn get_epic_images_from(
    base_url: &str,
    api_key: String,
    kind: &str,
    date: Option<&str>,
) -> Result<Vec<EpicImage>, Error> {
    let base = base_url.trim_end_matches('/');
    let endpoint = match date {
        Some(date) => format!("{base}/api/{kind}/date/{date}"),
        None => format!("{base}/api/{kind}/images"),
    };
    let query_key = api_key.clone();
    let archive_key = api_key;
    let kind = kind.to_string();

    let metadata: Vec<EpicImageMetadata> = http::get_json(SERVICE_NAME, move || {
        http::client()
            .get(&endpoint)
            .query(&[("api_key", query_key.clone())])
    })
    .await?;

    Ok(metadata
        .into_iter()
        .map(|item| EpicImage {
            url: archive_url(&kind, &item.date, &item.image, &archive_key),
            image: item.image,
            caption: item.caption,
            date: item.date,
        })
        .collect())
}

/// Fetches EPIC images from NASA.
pub async fn get_epic_images(
    api_key: String,
    kind: &str,
    date: Option<&str>,
) -> Result<Vec<EpicImage>, Error> {
    let endpoint = endpoint(kind, date);
    let query_key = api_key.clone();
    let archive_key = api_key;
    let kind = kind.to_string();

    let metadata: Vec<EpicImageMetadata> = http::get_json(SERVICE_NAME, move || {
        http::client()
            .get(&endpoint)
            .query(&[("api_key", query_key.clone())])
    })
    .await?;

    Ok(metadata
        .into_iter()
        .map(|item| EpicImage {
            url: archive_url(&kind, &item.date, &item.image, &archive_key),
            image: item.image,
            caption: item.caption,
            date: item.date,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{archive_url, get_epic_images_from, EpicImage};
    use crate::test_utils::spawn_json_server;

    #[tokio::test]
    async fn parses_epic_response() {
        let server = spawn_json_server(
            r#"[{"image":"epic_1b_20260409011359","caption":"Earth","date":"2026-04-09 01:13:59"}]"#,
        )
        .await;

        let response = get_epic_images_from(&server.url, "abc123".to_string(), "natural", None)
            .await
            .expect("epic response");

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].image, "epic_1b_20260409011359");
        assert_eq!(response[0].caption, "Earth");
        assert_eq!(response[0].short_date(), "2026-04-09");
        assert_eq!(
            response[0].url,
            "https://api.nasa.gov/EPIC/archive/natural/2026/04/09/png/epic_1b_20260409011359.png?api_key=abc123"
        );
        assert_eq!(
            server.request_line().await.as_deref(),
            Some("GET /api/natural/images?api_key=abc123 HTTP/1.1")
        );
    }

    #[test]
    fn builds_archive_url() {
        assert_eq!(
            archive_url(
                "enhanced",
                "2026-04-09 01:13:59",
                "epic_1b_20260409011359",
                "abc123"
            ),
            "https://api.nasa.gov/EPIC/archive/enhanced/2026/04/09/png/epic_1b_20260409011359.png?api_key=abc123"
        );
    }

    #[test]
    fn epic_image_short_date_uses_day_prefix() {
        let image = EpicImage {
            image: "epic".to_string(),
            caption: "Earth".to_string(),
            date: "2026-04-09 01:13:59".to_string(),
            url: "https://example.com".to_string(),
        };

        assert_eq!(image.short_date(), "2026-04-09");
    }
}
