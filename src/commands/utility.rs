use std::time::Duration;

use crate::types::{Context, Error};

/// Shares runtime metadata for the bot.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra informações do bot")
)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "about");

    let version = env!("CARGO_PKG_VERSION");
    let message = format!(
        "S-sou a Seris v{version}\nConheço estes comandos: ping, clear, epic, apod, anime, manga, about, uptime, stats"
    );

    ctx.say(message).await?;
    Ok(())
}

/// Reports how long the bot has been running.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra há quanto tempo o bot está ligado")
)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "uptime");

    let uptime = ctx.data().started_at.elapsed();
    let message = format!("Estou ativa há {}...", format_duration(uptime));

    ctx.say(message).await?;
    Ok(())
}

/// Shows persisted command usage statistics.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra estatísticas persistidas")
)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let summary = match ctx.data().database.command_usage_summary(ctx.author().id) {
        Ok(summary) => summary,
        Err(err) => {
            log::error!("failed to load persisted stats: {err}");
            ctx.say("E-eu não consegui ler suas estatísticas agora.")
                .await?;
            return Ok(());
        }
    };

    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "stats");

    let message = if let Some(favorite_command) = summary.favorite_command {
        format!(
            "Pelo que vi, você usou {total} comandos em {distinct} comandos diferentes. Favorito: `/{favorite}` ({count} vezes)...",
            total = summary.total_uses,
            distinct = summary.distinct_commands,
            favorite = favorite_command,
            count = summary.favorite_count,
        )
    } else {
        "Ainda não tenho comandos salvos para você...".to_string()
    };

    ctx.say(message).await?;
    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        format!("{days}d {hours:02}h {minutes:02}m {seconds:02}s")
    } else if hours > 0 {
        format!("{hours}h {minutes:02}m {seconds:02}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds:02}s")
    } else {
        format!("{seconds}s")
    }
}

#[cfg(test)]
mod tests {
    use super::format_duration;
    use std::time::Duration;

    #[test]
    fn formats_uptime_compactly() {
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
        assert_eq!(format_duration(Duration::from_secs(61)), "1m 01s");
        assert_eq!(format_duration(Duration::from_secs(3_661)), "1h 01m 01s");
        assert_eq!(
            format_duration(Duration::from_secs(90_061)),
            "1d 01h 01m 01s"
        );
    }
}
