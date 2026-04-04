//! Jikan API helpers and response models.

use reqwest::Client;
use serde::Deserialize;

use crate::types::Error;

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

fn get_api_url() -> String {
    "https://api.jikan.moe/v4".to_string()
}

/// Fetches a random anime from Jikan.
pub async fn get_random_anime() -> Result<Response<Anime>, Error> {
    let client = Client::new();

    let response = client
        .get(format!("{}/random/anime", get_api_url()))
        .send()
        .await?;

    Ok(response.json::<Response<Anime>>().await?)
}

/// Fetches a random manga from Jikan.
pub async fn get_random_manga() -> Result<Response<Manga>, Error> {
    let client = Client::new();

    let response = client
        .get(format!("{}/random/manga", get_api_url()))
        .send()
        .await?;

    Ok(response.json::<Response<Manga>>().await?)
}
