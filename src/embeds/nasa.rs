use serenity::{all::CreateEmbed, model::colour};

use crate::services::nasa::AstronomyPictureDay;

pub fn apod(data: AstronomyPictureDay) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title)
        .description(data.explanation)
        .image(data.hdurl)
        .color(colour::Colour::from_rgb(81, 78, 184))
}
