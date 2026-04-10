use serenity::{all::CreateEmbed, builder::CreateEmbedFooter, model::colour};

use crate::services::epic::EpicImage;

/// Builds an EPIC image embed.
pub fn epic(image: &EpicImage, page: usize, total: usize, kind: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("EPIC {kind}"))
        .url(image.url.clone())
        .description(image.caption.clone())
        .image(image.url.clone())
        .footer(CreateEmbedFooter::new(format!(
            "Página {}/{} • {} • {}",
            page + 1,
            total,
            image.short_date(),
            image.image
        )))
        .color(colour::Colour::from_rgb(47, 106, 217))
}

#[cfg(test)]
mod tests {
    use super::epic;
    use crate::services::epic::EpicImage;

    #[test]
    fn epic_sets_expected_fields() {
        let embed = epic(
            &EpicImage {
                image: "epic_1b_20260409011359".to_string(),
                caption: "Earth".to_string(),
                date: "2026-04-09 01:13:59".to_string(),
                url: "https://example.com/epic.png".to_string(),
            },
            0,
            3,
            "natural",
        );

        let value = serde_json::to_value(embed).expect("embed serializes");

        assert_eq!(value["title"], "EPIC natural");
        assert_eq!(value["description"], "Earth");
        assert_eq!(value["image"]["url"], "https://example.com/epic.png");
    }
}
