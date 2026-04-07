//! NASA API helpers and response models.

use chrono::Utc;
use reqwest::Url;
use serde::Deserialize;
use std::sync::{Mutex, OnceLock};

use super::http;
use crate::types::Error;

const API_URL: &str = "https://api.nasa.gov/planetary/apod";
const SERVICE_NAME: &str = "nasa-apod";

fn apod_url() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();
    URL.get_or_init(|| Url::parse(API_URL).expect("valid APOD url"))
}

#[derive(Clone)]
struct CachedApod {
    date: chrono::NaiveDate,
    data: AstronomyPictureDay,
}

fn cache() -> &'static Mutex<Option<CachedApod>> {
    static CACHE: OnceLock<Mutex<Option<CachedApod>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Fetches NASA's astronomy picture of the day from a specific base URL.
pub async fn get_astronomy_picture_day_from(
    base_url: &str,
    api_key: String,
) -> Result<AstronomyPictureDay, Error> {
    let url = base_url.to_string();
    let key = api_key;

    http::get_json(SERVICE_NAME, move || {
        http::client().get(&url).query(&[("api_key", key.clone())])
    })
    .await
}

#[cfg(test)]
async fn get_astronomy_picture_day_cached_from(
    base_url: &str,
    api_key: String,
) -> Result<AstronomyPictureDay, Error> {
    let today = Utc::now().date_naive();

    if let Some(entry) = cache()
        .lock()
        .expect("apod cache lock")
        .as_ref()
        .filter(|entry| entry.date == today)
        .cloned()
    {
        return Ok(entry.data);
    }

    let data = get_astronomy_picture_day_from(base_url, api_key).await?;
    *cache().lock().expect("apod cache lock") = Some(CachedApod {
        date: today,
        data: data.clone(),
    });

    Ok(data)
}

/// NASA APOD response payload.
#[derive(Clone, Deserialize, Debug)]
pub struct AstronomyPictureDay {
    /// APOD explanation text.
    pub explanation: String,
    /// High-resolution image URL.
    pub hdurl: String,
    /// APOD title.
    pub title: String,
}

/// Fetches NASA's astronomy picture of the day.
pub async fn get_astronomy_picture_day(api_key: String) -> Result<AstronomyPictureDay, Error> {
    let today = Utc::now().date_naive();

    if let Some(entry) = cache()
        .lock()
        .expect("apod cache lock")
        .as_ref()
        .filter(|entry| entry.date == today)
        .cloned()
    {
        return Ok(entry.data);
    }

    let key = api_key;
    let data: AstronomyPictureDay = http::get_json(SERVICE_NAME, || {
        http::client()
            .get(apod_url().clone())
            .query(&[("api_key", key.clone())])
    })
    .await?;

    *cache().lock().expect("apod cache lock") = Some(CachedApod {
        date: today,
        data: data.clone(),
    });

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::{get_astronomy_picture_day_cached_from, get_astronomy_picture_day_from};
    use crate::test_utils::spawn_scripted_server;
    use crate::test_utils::{spawn_json_server, TestResponse};

    #[tokio::test]
    async fn parses_apod_response() {
        let server = spawn_json_server(
            r#"{"title":"APOD","explanation":"Space","hdurl":"https://example.com/apod.jpg"}"#,
        )
        .await;

        let response = get_astronomy_picture_day_from(&server.url, "abc123".to_string())
            .await
            .expect("apod response");

        assert_eq!(response.title, "APOD");
        assert_eq!(response.explanation, "Space");
        assert_eq!(response.hdurl, "https://example.com/apod.jpg");
        assert_eq!(
            server.request_line().await.as_deref(),
            Some("GET /?api_key=abc123 HTTP/1.1")
        );
    }

    #[tokio::test]
    async fn caches_apod_response_for_the_same_day() {
        let server = spawn_scripted_server(vec![TestResponse::new(
            200,
            r#"{"title":"Cached","explanation":"Space","hdurl":"https://example.com/cached.jpg"}"#,
        )])
        .await;

        let first = get_astronomy_picture_day_cached_from(&server.url, "abc123".to_string())
            .await
            .expect("first apod response");
        let second = get_astronomy_picture_day_cached_from(&server.url, "abc123".to_string())
            .await
            .expect("second apod response");

        assert_eq!(first.title, "Cached");
        assert_eq!(second.title, "Cached");
        assert_eq!(server.request_count().await, 1);
    }
}
