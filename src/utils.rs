//! Serenity event handling utilities.

use log::info;
use serenity::{
    all::{Context, EventHandler, Ready, ResumedEvent},
    async_trait,
    gateway::ShardStageUpdateEvent,
};

use crate::health::{apply_stage_update as apply_health_stage_update, HealthState};

/// Event handler used by the bot client.
pub struct Handler {
    health: std::sync::Arc<HealthState>,
}

impl Handler {
    pub fn new(health: std::sync::Arc<HealthState>) -> Self {
        Self { health }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        self.health.mark_ready();
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Discord gateway resumed");
        self.health.mark_ready();
    }

    async fn shard_stage_update(&self, _: Context, event: ShardStageUpdateEvent) {
        apply_health_stage_update(&self.health, &event);
    }
}
