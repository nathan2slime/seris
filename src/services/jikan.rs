//! Jikan API helpers and response models.

use reqwest::Client;
use serde::Deserialize;

use crate::types::Error;

const API_URL: &str = "https://api.jikan.moe/v4";

/// Random anime payload returned by Jikan.
#[derive(Deserialize, Debug)]
pub struct Anime {
    /// Short synopsis of the anime.
    pub synopsis: String,
    /// Anime title.
    pub title: String,
    /// Cover images returned by Jikan.
    pub images: Images,
}

/// Random manga payload returned by Jikan.
#[derive(Deserialize, Debug)]
pub struct Manga {
    /// Short synopsis of the manga.
    pub synopsis: String,
    /// Manga title.
    pub title: String,
    /// Cover images returned by Jikan.
    pub images: Images,
}

/// Image metadata returned by Jikan.
#[derive(Deserialize, Debug)]
pub struct Image {
    /// Direct image URL.
    pub image_url: String,
}

/// Image variants returned by Jikan.
#[derive(Deserialize, Debug)]
pub struct Images {
    /// JPG image variant.
    pub jpg: Image,
}

/// Generic Jikan response wrapper.
#[derive(Deserialize, Debug)]
pub struct Response<D> {
    /// Response payload.
    pub data: D,
}

fn get_api_url() -> &'static str {
    API_URL
}

/// Fetches a random anime from a specific Jikan base URL.
pub async fn get_random_anime_from(base_url: &str) -> Result<Response<Anime>, Error> {
    let client = Client::new();

    let response = client
        .get(format!("{}/random/anime", base_url))
        .send()
        .await?;

    Ok(response.json::<Response<Anime>>().await?)
}

/// Fetches a random manga from a specific Jikan base URL.
pub async fn get_random_manga_from(base_url: &str) -> Result<Response<Manga>, Error> {
    let client = Client::new();

    let response = client
        .get(format!("{}/random/manga", base_url))
        .send()
        .await?;

    Ok(response.json::<Response<Manga>>().await?)
}

/// Fetches a random anime from Jikan.
pub async fn get_random_anime() -> Result<Response<Anime>, Error> {
    get_random_anime_from(get_api_url()).await
}

/// Fetches a random manga from Jikan.
pub async fn get_random_manga() -> Result<Response<Manga>, Error> {
    get_random_manga_from(get_api_url()).await
}

#[cfg(test)]
mod tests {
    use super::{get_random_anime_from, get_random_manga_from};
    use crate::test_utils::spawn_json_server;

    #[tokio::test]
    async fn parses_random_anime_response() {
        let server = spawn_json_server(
            r#"{"data":{"title":"Anime","synopsis":"Synopsis","images":{"jpg":{"image_url":"https://example.com/anime.jpg"}}}}"#,
        )
        .await;

        let response = get_random_anime_from(&server.url)
            .await
            .expect("anime response");

        assert_eq!(response.data.title, "Anime");
        assert_eq!(response.data.synopsis, "Synopsis");
        assert_eq!(
            response.data.images.jpg.image_url,
            "https://example.com/anime.jpg"
        );
        assert_eq!(
            server.request_line().await.as_deref(),
            Some("GET /random/anime HTTP/1.1")
        );
    }

    #[tokio::test]
    async fn parses_random_manga_response() {
        let server = spawn_json_server(
            r#"{"data":{"title":"Manga","synopsis":"Synopsis","images":{"jpg":{"image_url":"https://example.com/manga.jpg"}}}}"#,
        )
        .await;

        let response = get_random_manga_from(&server.url)
            .await
            .expect("manga response");

        assert_eq!(response.data.title, "Manga");
        assert_eq!(response.data.synopsis, "Synopsis");
        assert_eq!(
            response.data.images.jpg.image_url,
            "https://example.com/manga.jpg"
        );
        assert_eq!(
            server.request_line().await.as_deref(),
            Some("GET /random/manga HTTP/1.1")
        );
    }
}
