use log::{error, info};
use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::nasa::get_astronomy_picture_day;

/// Replies with NASA's astronomy picture or video of the day.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Imagem ou Vídeo Astronômico do Dia")
)]
pub async fn apod(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "apod");

    let nasa_api_key = ctx.data().config.nasa_api_key.clone();
    let res = get_astronomy_picture_day(nasa_api_key).await;

    match res {
        Ok(data) => {
            let mut reply = CreateReply::default().embed(embeds::nasa::apod(&data));

            if data.is_video() {
                info!("APOD returned video: {}", data.title);
                reply = reply.content(data.url);
            } else {
                info!("APOD returned image: {}", data.title);
            }

            ctx.send(reply).await?;
        }
        Err(err) => {
            error!("failed to fetch APOD: {err}");
            ctx.say("Algo deu errado. Tente novamente mais tarde!")
                .await?;
        }
    };

    Ok(())
}
