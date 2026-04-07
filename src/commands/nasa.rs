use std::time::Duration;

use poise::CreateReply;

use crate::{
    cooldown,
    types::{Context, Error},
};

use crate::embeds;
use crate::services::nasa::get_astronomy_picture_day;

const APOD_COOLDOWN: Duration = Duration::from_secs(20);

/// Replies with NASA's astronomy picture of the day.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Imagem Astronômica do Dia")
)]
pub async fn apod(ctx: Context<'_>) -> Result<(), Error> {
    if !cooldown::enforce(&ctx, "apod", APOD_COOLDOWN).await? {
        return Ok(());
    }

    let nasa_api_key = ctx.data().config.nasa_api_key.clone();
    let res = get_astronomy_picture_day(nasa_api_key).await;

    match res {
        Ok(data) => {
            ctx.send(CreateReply::default().embed(embeds::nasa::apod(data)))
                .await?;
        }
        Err(_err) => {
            log::error!("{:?}", _err);
            ctx.say("Algo deu errado. Tente novamente mais tarde!")
                .await?;
        }
    };

    Ok(())
}
