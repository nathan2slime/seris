//! Serenity event handling utilities.

use log::info;
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Event handler used by the bot client.
#[derive(Default)]
pub struct Handler {
    reminder_task_started: Arc<AtomicBool>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        if self
            .reminder_task_started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            tokio::spawn(crate::reminders::run_point_reminder_loop(ctx.http.clone()));
        }
    }
}
