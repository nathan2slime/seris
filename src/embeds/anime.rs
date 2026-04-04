use serenity::{all::CreateEmbed, model::colour};

use crate::services::jikan::Anime;

/// Builds the anime recommendation embed.
pub fn basic(data: Anime) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title)
        .description(data.synopsis)
        .image(data.images.jpg.image_url)
        .color(colour::Colour::from_rgb(81, 78, 184))
}

#[cfg(test)]
mod tests {
    use super::basic;
    use crate::services::jikan::{Anime, Image, Images};

    #[test]
    fn basic_sets_expected_fields() {
        let embed = basic(Anime {
            title: "Title".to_string(),
            synopsis: "Synopsis".to_string(),
            images: Images {
                jpg: Image {
                    image_url: "https://example.com/anime.jpg".to_string(),
                },
            },
        });

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["title"], "Title");
        assert_eq!(value["description"], "Synopsis");
        assert_eq!(value["image"]["url"], "https://example.com/anime.jpg");
    }
}
