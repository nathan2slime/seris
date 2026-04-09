//! Serenity event handling utilities.

use log::info;
use serenity::{
    all::{Context, EventHandler, Ready, ResumedEvent},
    async_trait,
};

/// Event handler used by the bot client.
#[derive(Default)]
pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Discord gateway resumed");
    }
}
