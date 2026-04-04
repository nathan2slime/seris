//! NASA API helpers and response models.

use reqwest::Client;
use serde::Deserialize;

use crate::types::Error;

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
pub async fn get_astronomy_picture_day(api_key: String) -> Result<AstronomyPictureDay, Error> {
    let client = Client::new();

    let url = "https://api.nasa.gov/planetary/apod";
    let params = [("api_key", api_key)];

    let response = client.get(url).query(&params).send().await?;

    Ok(response.json::<AstronomyPictureDay>().await?)
}
