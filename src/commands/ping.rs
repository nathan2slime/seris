use crate::types::{Context, Error};

/// Replies with a gentle Pong.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Responde baixinho com Pong")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "ping");

    let author = ctx.author();

    ctx.say(format!("P-pong... <@{}>! 🐾✨", author.id)).await?;

    Ok(())
}
