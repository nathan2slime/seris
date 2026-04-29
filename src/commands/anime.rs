use log::error;
use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::jikan;

/// Replies with a random anime recommendation.
#[poise::command(
    slash_command,
    rename = "get_random_anime",
    description_localized("pt-BR", "Posso sugerir um anime")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "get_random_anime");

    ctx.defer().await?;

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

            ctx.say("E-eu falhei... tente novamente mais tarde.")
                .await?;
        }
    };

    Ok(())
}
