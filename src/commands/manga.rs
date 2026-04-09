use log::error;
use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::jikan;

/// Replies with a random manga recommendation.
#[poise::command(
    slash_command,
    rename = "get_random_manga",
    description_localized("pt-BR", "Posso sugerir um mangá")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "get_random_manga");

    ctx.defer().await?;

    let response = jikan::get_random_manga().await;

    match response {
        Ok(res) => {
            ctx.send(
                CreateReply::default()
                    .embed(embeds::manga::basic(res.data))
                    .content("T-tem um mangá que talvez te agrade..."),
            )
            .await?;
        }
        Err(_err) => {
            error!("{:?}", _err);

            ctx.say("D-desculpe... algo deu errado").await?;
        }
    };

    Ok(())
}
