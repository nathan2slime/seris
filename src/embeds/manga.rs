use serenity::{all::CreateEmbed, model::colour};

use crate::services::jikan::Manga;

pub fn basic(data: Manga) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title)
        .description(data.synopsis)
        .image(data.images.jpg.image_url)
        .color(colour::Colour::from_rgb(81, 78, 184))
}
