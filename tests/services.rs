mod common;

use common::spawn_json_server;
use seris::services::jikan::{get_random_anime_from, get_random_manga_from};
use seris::services::nasa::get_astronomy_picture_day_from;

#[tokio::test]
async fn fetches_anime_from_mock_server() {
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
async fn fetches_manga_from_mock_server() {
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

#[tokio::test]
async fn fetches_apod_from_mock_server() {
    let server = spawn_json_server(
        r#"{"title":"APOD","explanation":"Space","media_type":"image","url":"https://example.com/apod.jpg","hdurl":"https://example.com/apod.jpg"}"#,
    )
    .await;

    let response = get_astronomy_picture_day_from(&server.url, "abc123".to_string())
        .await
        .expect("apod response");

    assert_eq!(response.title, "APOD");
    assert_eq!(response.explanation, "Space");
    assert_eq!(response.media_type, "image");
    assert_eq!(response.url, "https://example.com/apod.jpg");
    assert_eq!(
        response.hdurl.as_deref(),
        Some("https://example.com/apod.jpg")
    );
    assert_eq!(
        server.request_line().await.as_deref(),
        Some("GET /?api_key=abc123 HTTP/1.1")
    );
}

#[tokio::test]
async fn fetches_video_apod_from_mock_server() {
    let server = spawn_json_server(
        r#"{"title":"APOD","explanation":"Space","media_type":"video","url":"https://example.com/apod.mp4"}"#,
    )
    .await;

    let response = get_astronomy_picture_day_from(&server.url, "abc123".to_string())
        .await
        .expect("apod response");

    assert_eq!(response.media_type, "video");
    assert_eq!(response.url, "https://example.com/apod.mp4");
    assert_eq!(response.hdurl, None);
}

#[tokio::test]
async fn rejects_invalid_json() {
    let server = spawn_json_server("not-json").await;

    assert!(get_random_anime_from(&server.url).await.is_err());
}
