use serenity::{all::CreateEmbed, model::colour};

use crate::services::nasa::AstronomyPictureDay;

/// Builds the APOD embed.
pub fn apod(data: AstronomyPictureDay) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title)
        .description(data.explanation)
        .image(data.hdurl)
        .color(colour::Colour::from_rgb(81, 78, 184))
}

#[cfg(test)]
mod tests {
    use super::apod;
    use crate::services::nasa::AstronomyPictureDay;

    #[test]
    fn apod_sets_expected_fields() {
        let embed = apod(AstronomyPictureDay {
            title: "APOD".to_string(),
            explanation: "Space".to_string(),
            hdurl: "https://example.com/apod.jpg".to_string(),
        });

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["title"], "APOD");
        assert_eq!(value["description"], "Space");
        assert_eq!(value["image"]["url"], "https://example.com/apod.jpg");
    }
}
