use std::time::Duration;

use crate::types::{Context, Error};

/// Shares runtime metadata for the bot.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra informações do bot")
)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let version = env!("CARGO_PKG_VERSION");
    let message =
        format!("Seris v{version}\nSlash commands: ping, clear, apod, anime, manga, about, uptime");

    ctx.say(message).await?;
    Ok(())
}

/// Reports how long the bot has been running.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra há quanto tempo o bot está ligado")
)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = ctx.data().started_at.elapsed();
    let message = format!("Seris está ativa há {}", format_duration(uptime));

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
