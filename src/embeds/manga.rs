use serenity::{all::CreateEmbed, model::colour};

use crate::services::jikan::Manga;

/// Builds the manga recommendation embed.
pub fn basic(data: Manga) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title)
        .description(data.synopsis)
        .image(data.images.jpg.image_url)
        .color(colour::Colour::from_rgb(81, 78, 184))
}

#[cfg(test)]
mod tests {
    use super::basic;
    use crate::services::jikan::{Image, Images, Manga};

    #[test]
    fn basic_sets_expected_fields() {
        let embed = basic(Manga {
            title: "Title".to_string(),
            synopsis: "Synopsis".to_string(),
            images: Images {
                jpg: Image {
                    image_url: "https://example.com/manga.jpg".to_string(),
                },
            },
        });

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["title"], "Title");
        assert_eq!(value["description"], "Synopsis");
        assert_eq!(value["image"]["url"], "https://example.com/manga.jpg");
    }
}
