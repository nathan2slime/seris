use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Anime {
    pub synopsis: String,
    pub title: String,
    pub images: Images, 
}


#[derive(Deserialize, Debug)]
pub struct Manga {
    pub synopsis: String,
    pub title: String,
    pub images: Images, 
}


#[derive(Deserialize, Debug)]
pub struct Image {
    pub image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub jpg: Image,
}

#[derive(Deserialize, Debug)]
pub struct Response<D> {
    pub data: D,
}

fn get_api_url() -> String {
    "https://api.jikan.moe/v4".to_string()
}

pub async fn get_random_anime() -> Result<Response<Anime>, Error> {
    let client = Client::new();

    let response = client.get(format!("{}/random/anime", get_api_url())).send().await;

    match response {
        Ok(res) => {
            let data = res.json::<Response<Anime>>().await?;

            Ok(data)
        }
        Err(err) => return Err(err),
    }
}

pub async fn get_random_manga() -> Result<Response<Manga>, Error> {
    let client = Client::new();

    let response = client.get(format!("{}/random/manga", get_api_url())).send().await;

    match response {
        Ok(res) => {
            let data = res.json::<Response<Manga>>().await?;

            Ok(data)
        }
        Err(err) => return Err(err),
    }
}

