//! Serenity event handling utilities.

use log::info;
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

/// Event handler used by the bot client.
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
