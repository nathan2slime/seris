//! Simple per-user command cooldown tracking.

use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use dashmap::{mapref::entry::Entry, DashMap};
use serenity::all::UserId;

use crate::types::{Context, Error};

struct CooldownTracker {
    entries: DashMap<(&'static str, UserId), Instant>,
}

impl CooldownTracker {
    fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    fn try_acquire(
        &self,
        command: &'static str,
        user_id: UserId,
        cooldown: Duration,
        now: Instant,
    ) -> Option<Duration> {
        let expires_at = now + cooldown;

        match self.entries.entry((command, user_id)) {
            Entry::Occupied(mut occupied) => {
                let current = *occupied.get();

                if current > now {
                    Some(current - now)
                } else {
                    occupied.insert(expires_at);
                    None
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(expires_at);
                None
            }
        }
    }
}

fn tracker() -> &'static CooldownTracker {
    static TRACKER: OnceLock<CooldownTracker> = OnceLock::new();
    TRACKER.get_or_init(CooldownTracker::new)
}

/// Checks whether the user can run the command and sends a cooldown warning when blocked.
pub async fn enforce(
    ctx: &Context<'_>,
    command: &'static str,
    cooldown: Duration,
) -> Result<bool, Error> {
    if let Some(wait) = tracker().try_acquire(command, ctx.author().id, cooldown, Instant::now()) {
        ctx.say(format!(
            "Aguarde {} antes de usar `/{command}` novamente.",
            format_duration(wait)
        ))
        .await?;

        return Ok(false);
    }

    Ok(true)
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
    use super::CooldownTracker;
    use serenity::all::UserId;
    use std::time::{Duration, Instant};

    #[test]
    fn allows_first_request() {
        let tracker = CooldownTracker::new();

        assert_eq!(
            tracker.try_acquire(
                "ping",
                UserId::new(1),
                Duration::from_secs(5),
                Instant::now()
            ),
            None
        );
    }

    #[test]
    fn blocks_repeat_request_until_expired() {
        let tracker = CooldownTracker::new();
        let now = Instant::now();
        let user_id = UserId::new(1);

        assert_eq!(
            tracker.try_acquire("ping", user_id, Duration::from_secs(5), now),
            None
        );

        let remaining = tracker
            .try_acquire(
                "ping",
                user_id,
                Duration::from_secs(5),
                now + Duration::from_secs(2),
            )
            .expect("cooldown");

        assert!(remaining <= Duration::from_secs(3));
    }

    #[test]
    fn different_commands_have_independent_cooldowns() {
        let tracker = CooldownTracker::new();
        let now = Instant::now();
        let user_id = UserId::new(1);

        assert_eq!(
            tracker.try_acquire("ping", user_id, Duration::from_secs(5), now),
            None
        );
        assert_eq!(
            tracker.try_acquire("uptime", user_id, Duration::from_secs(5), now),
            None
        );
    }
}
