use crate::types::{Context, Error};

/// Replies with a friendly Pong.
#[poise::command(slash_command, description_localized("pt-BR", "Responde com Pong!"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();

    ctx.say(format!("Yay!, <@{}>! 🐾✨ Pong! 🎉", author.id))
        .await?;

    Ok(())
}
