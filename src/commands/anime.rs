use log::error;
use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::jikan;

#[poise::command(
    slash_command,
    rename = "get_random_anime",
    description_localized("pt-BR", "Recomendo um anime")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
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
