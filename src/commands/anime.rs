use log::error;
use poise::CreateReply;
use std::time::Duration;

use crate::{
    cooldown,
    types::{Context, Error},
};

use crate::embeds;
use crate::services::jikan;

const ANIME_COOLDOWN: Duration = Duration::from_secs(20);

/// Replies with a random anime recommendation.
#[poise::command(
    slash_command,
    rename = "get_random_anime",
    description_localized("pt-BR", "Recomendo um anime")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    if !cooldown::enforce(&ctx, "get_random_anime", ANIME_COOLDOWN).await? {
        return Ok(());
    }

    let response = jikan::get_random_anime().await;

    match response {
        Ok(res) => {
            ctx.send(
                CreateReply::default()
                    .embed(embeds::anime::basic(res.data))
                    .content(""),
            )
            .await?;
        }
        Err(_err) => {
            error!("{:?}", _err);

            ctx.say("Algo deu errado. Tente novamente mais tarde!")
                .await?;
        }
    };

    Ok(())
}
