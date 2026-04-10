use log::{error, info};
use poise::CreateReply;

use crate::embeds;
use crate::epic::EpicSession;
use crate::services::epic::get_epic_images;
use crate::types::{Context, Error};

/// Replies with paginated EPIC Earth imagery.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra imagens EPIC da Terra com botões")
)]
pub async fn epic(
    ctx: Context<'_>,
    #[choices("natural", "enhanced")]
    #[description = "Tipo de imagem EPIC"]
    kind: &'static str,
    #[description = "Data YYYY-MM-DD (opcional)"] date: Option<String>,
) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "epic");

    let normalized_date = match date {
        Some(date) => {
            if chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").is_err() {
                ctx.say("D-desculpe... a data precisa estar no formato YYYY-MM-DD.")
                    .await?;
                return Ok(());
            }

            Some(date)
        }
        None => None,
    };

    ctx.defer().await?;

    let nasa_api_key = ctx.data().config.nasa_api_key.clone();
    let images = get_epic_images(nasa_api_key, kind, normalized_date.as_deref()).await;

    let images = match images {
        Ok(images) if !images.is_empty() => images,
        Ok(_) => {
            ctx.say("E-eu não encontrei imagens EPIC para essa data.")
                .await?;
            return Ok(());
        }
        Err(err) => {
            error!("failed to fetch EPIC images: {err}");
            ctx.say("D-desculpe... tente novamente mais tarde.").await?;
            return Ok(());
        }
    };

    info!("EPIC returned {} images for {}", images.len(), kind);

    let session = EpicSession::new(ctx.author().id, kind.to_string(), images);
    let reply = CreateReply::default()
        .embed(embeds::epic::epic(
            session.current_image(),
            session.current_index(),
            session.total_pages(),
            session.kind(),
        ))
        .components(session.components());

    let handle = ctx.send(reply).await?;
    let message = handle.message().await?;
    ctx.data().epic_sessions.insert(message.id.get(), session);

    Ok(())
}
