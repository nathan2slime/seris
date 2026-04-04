//! NASA API helpers and response models.

use reqwest::Client;
use serde::Deserialize;

use crate::types::Error;

const API_URL: &str = "https://api.nasa.gov/planetary/apod";

/// NASA APOD response payload.
#[derive(Deserialize, Debug)]
pub struct AstronomyPictureDay {
    /// APOD explanation text.
    pub explanation: String,
    /// High-resolution image URL.
    pub hdurl: String,
    /// APOD title.
    pub title: String,
}

/// Fetches NASA's astronomy picture of the day.
async fn get_astronomy_picture_day_from(
    base_url: &str,
    api_key: String,
) -> Result<AstronomyPictureDay, Error> {
    let client = Client::new();

    let params = [("api_key", api_key)];

    let response = client.get(base_url).query(&params).send().await?;

    Ok(response.json::<AstronomyPictureDay>().await?)
}

/// Fetches NASA's astronomy picture of the day.
pub async fn get_astronomy_picture_day(api_key: String) -> Result<AstronomyPictureDay, Error> {
    get_astronomy_picture_day_from(API_URL, api_key).await
}

#[cfg(test)]
mod tests {
    use super::get_astronomy_picture_day_from;
    use crate::test_utils::spawn_json_server;

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
}
