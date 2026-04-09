use serenity::{all::CreateEmbed, model::colour};

use crate::services::nasa::AstronomyPictureDay;

/// Builds the APOD embed.
pub fn apod(data: &AstronomyPictureDay) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title(data.title.clone())
        .description(data.explanation.clone())
        .color(colour::Colour::from_rgb(81, 78, 184));

    if data.is_video() {
        embed
            .url(data.url.clone())
            .field("Video", format!("[Abrir vídeo]({})", data.url), false)
    } else {
        embed.image(data.image_url())
    }
}

#[cfg(test)]
mod tests {
    use super::apod;
    use crate::services::nasa::AstronomyPictureDay;

    #[test]
    fn apod_sets_expected_fields() {
        let embed = apod(&AstronomyPictureDay {
            title: "APOD".to_string(),
            explanation: "Space".to_string(),
            media_type: "image".to_string(),
            url: "https://example.com/apod.jpg".to_string(),
            hdurl: Some("https://example.com/apod.jpg".to_string()),
        });

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["title"], "APOD");
        assert_eq!(value["description"], "Space");
        assert_eq!(value["image"]["url"], "https://example.com/apod.jpg");
    }

    #[test]
    fn apod_links_video_media() {
        let embed = apod(&AstronomyPictureDay {
            title: "APOD".to_string(),
            explanation: "Space".to_string(),
            media_type: "video".to_string(),
            url: "https://example.com/apod.mp4".to_string(),
            hdurl: None,
        });

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["url"], "https://example.com/apod.mp4");
        assert_eq!(value["fields"][0]["name"], "Video");
    }
}
