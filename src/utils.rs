//! Serenity event handling utilities.

use log::info;
use serenity::{
    all::{Context, EventHandler, Ready, ResumedEvent},
    async_trait,
    gateway::ShardStageUpdateEvent,
};

use crate::dashboard::{apply_stage_update, DashboardState};

/// Event handler used by the bot client.
pub struct Handler {
    dashboard: std::sync::Arc<DashboardState>,
}

impl Handler {
    pub fn new(dashboard: std::sync::Arc<DashboardState>) -> Self {
        Self { dashboard }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        self.dashboard.mark_ready();
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        self.dashboard.mark_ready();
    }

    async fn shard_stage_update(&self, _: Context, event: ShardStageUpdateEvent) {
        apply_stage_update(&self.dashboard, event.new);
    }
}
