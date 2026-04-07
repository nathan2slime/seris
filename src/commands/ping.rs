use std::time::Duration;

use crate::{
    cooldown,
    types::{Context, Error},
};

const PING_COOLDOWN: Duration = Duration::from_secs(5);

/// Replies with a friendly Pong.
#[poise::command(slash_command, description_localized("pt-BR", "Responde com Pong!"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    if !cooldown::enforce(&ctx, "ping", PING_COOLDOWN).await? {
        return Ok(());
    }

    let author = ctx.author();

    ctx.say(format!("Yay!, <@{}>! 🐾✨ Pong! 🎉", author.id))
        .await?;

    Ok(())
}
