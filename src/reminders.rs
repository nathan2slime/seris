//! Scheduled DM reminders for time tracking.

use chrono::{
    DateTime, Duration as ChronoDuration, Local, LocalResult, NaiveDateTime, NaiveTime, TimeZone,
    Timelike,
};
use log::{info, warn};
use serenity::{
    all::{CreateMessage, User},
    http::Http,
};
use std::{sync::Arc, time::Duration};

const KAWAII_REMINDER_GIFS: [&str; 3] = [
    "https://media.tenor.com/KDzt7A8t8WQAAAAC/anime-girl.gif",
    "https://media.tenor.com/6kJd8PmdJxQAAAAC/anime-kawaii.gif",
    "https://media.tenor.com/2roX3uxz_68AAAAC/anime-cute.gif",
];

fn reminder_times() -> [NaiveTime; 3] {
    [
        NaiveTime::from_hms_opt(12, 0, 0).expect("valid reminder time"),
        NaiveTime::from_hms_opt(13, 5, 0).expect("valid reminder time"),
        NaiveTime::from_hms_opt(17, 53, 0).expect("valid reminder time"),
    ]
}

fn next_reminder_after_naive(now: NaiveDateTime) -> NaiveDateTime {
    for time in reminder_times() {
        let candidate = now.date().and_time(time);
        if candidate > now {
            return candidate;
        }
    }

    let tomorrow = now
        .date()
        .checked_add_signed(ChronoDuration::days(1))
        .expect("next day should be representable");

    tomorrow.and_time(reminder_times()[0])
}

fn localize(naive: NaiveDateTime) -> DateTime<Local> {
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(datetime) | LocalResult::Ambiguous(datetime, _) => datetime,
        LocalResult::None => {
            let fallback = naive + ChronoDuration::hours(1);
            localize(fallback)
        }
    }
}

fn next_reminder_after(now: DateTime<Local>) -> DateTime<Local> {
    localize(next_reminder_after_naive(now.naive_local()))
}

fn sleep_duration_until(target: DateTime<Local>, now: DateTime<Local>) -> Duration {
    (target - now).to_std().unwrap_or(Duration::ZERO)
}

async fn fetch_application_owner(http: &Http) -> serenity::Result<User> {
    let application = http.get_current_application_info().await?;

    if let Some(owner) = application.owner {
        return Ok(owner);
    }

    if let Some(team) = application.team {
        return team.owner_user_id.to_user(http).await;
    }

    Err(serenity::Error::Other("application owner is unavailable"))
}

async fn send_point_reminder(
    http: &Http,
    owner: &User,
    target: DateTime<Local>,
) -> serenity::Result<()> {
    let gif_url =
        KAWAII_REMINDER_GIFS[(target.time().hour() as usize) % KAWAII_REMINDER_GIFS.len()];

    owner
        .direct_message(
            http,
            CreateMessage::new().content(format!(
                "Onii-chan, chegou a horinha de marcar o pontinho das {} kawaii~\nNao esquece, ta bom? Eu vou ficar felizinha quando voce marcar certinho.\n{}",
                target.format("%H:%M"),
                gif_url,
            )),
        )
        .await?;

    Ok(())
}

pub async fn run_point_reminder_loop(http: Arc<Http>) {
    let owner = match fetch_application_owner(http.as_ref()).await {
        Ok(owner) => owner,
        Err(err) => {
            warn!("point reminders disabled: could not resolve application owner: {err}");
            return;
        }
    };

    info!("point reminders enabled for application owner {}", owner.id);

    loop {
        let now = Local::now();
        let target = next_reminder_after(now);
        let sleep_for = sleep_duration_until(target, now);

        info!(
            "next point reminder scheduled for {}",
            target.format("%Y-%m-%d %H:%M:%S")
        );

        tokio::time::sleep(sleep_for).await;

        if let Err(err) = send_point_reminder(http.as_ref(), &owner, target).await {
            warn!("failed to send point reminder DM: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::next_reminder_after_naive;
    use chrono::{NaiveDate, NaiveDateTime};

    fn datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .expect("valid date")
            .and_hms_opt(hour, minute, 0)
            .expect("valid time")
    }

    #[test]
    fn picks_first_remaining_time_today() {
        let now = datetime(2026, 4, 29, 11, 30);

        assert_eq!(next_reminder_after_naive(now), datetime(2026, 4, 29, 12, 0));
    }

    #[test]
    fn skips_current_time_and_moves_to_next_slot() {
        let now = datetime(2026, 4, 29, 12, 0);

        assert_eq!(next_reminder_after_naive(now), datetime(2026, 4, 29, 13, 5));
    }

    #[test]
    fn rolls_over_to_next_day_after_last_slot() {
        let now = datetime(2026, 4, 29, 18, 0);

        assert_eq!(next_reminder_after_naive(now), datetime(2026, 4, 30, 12, 0));
    }
}
