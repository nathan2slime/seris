//! Health and readiness tracking for the bot runtime.

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use log::{info, warn};
use serenity::gateway::{ConnectionStage, ShardStageUpdateEvent};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::types::Error;

const HEALTH_ADDR: &str = "0.0.0.0:8080";

pub struct HealthState {
    ready: AtomicBool,
}

impl HealthState {
    pub fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
        }
    }

    pub fn mark_ready(&self) {
        self.ready.store(true, Ordering::SeqCst);
    }

    pub fn mark_not_ready(&self) {
        self.ready.store(false, Ordering::SeqCst);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }
}

impl Default for HealthState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn stage_is_ready(stage: &ConnectionStage) -> bool {
    matches!(stage, ConnectionStage::Connected)
}

pub fn status_for_path(path: &str, ready: bool) -> (u16, &'static str) {
    match path {
        "/health" => (200, "ok"),
        "/ready" if ready => (200, "ready"),
        "/ready" => (503, "not ready"),
        _ => (404, "not found"),
    }
}

pub async fn start(state: Arc<HealthState>) -> Result<(), Error> {
    let listener = TcpListener::bind(HEALTH_ADDR).await?;
    let addr = listener.local_addr()?;
    info!("health server listening on {addr}");

    loop {
        let (mut socket, peer) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            let Ok(bytes_read) =
                tokio::time::timeout(Duration::from_secs(2), socket.read(&mut buffer)).await
            else {
                return;
            };

            let Ok(bytes_read) = bytes_read else {
                return;
            };

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            let path = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or("/");

            let (status, body) = status_for_path(path, state.is_ready());
            let response = format!(
                "HTTP/1.1 {status} {status_text}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len(),
                status_text = status_text(status),
            );

            if let Err(err) = socket.write_all(response.as_bytes()).await {
                warn!("failed to write health response to {peer}: {err}");
            }
        });
    }
}

fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        404 => "Not Found",
        503 => "Service Unavailable",
        _ => "OK",
    }
}

pub fn apply_stage_update(state: &HealthState, event: &ShardStageUpdateEvent) {
    if stage_is_ready(&event.new) {
        state.mark_ready();
    } else {
        state.mark_not_ready();
    }
}

#[cfg(test)]
mod tests {
    use super::{stage_is_ready, status_for_path, HealthState};
    use serenity::gateway::ConnectionStage;

    #[test]
    fn health_route_is_always_ok() {
        assert_eq!(status_for_path("/health", false), (200, "ok"));
    }

    #[test]
    fn ready_route_reflects_readiness() {
        assert_eq!(status_for_path("/ready", false), (503, "not ready"));
        assert_eq!(status_for_path("/ready", true), (200, "ready"));
    }

    #[test]
    fn only_connected_stage_is_ready() {
        assert!(stage_is_ready(&ConnectionStage::Connected));
        assert!(!stage_is_ready(&ConnectionStage::Disconnected));
        assert!(!stage_is_ready(&ConnectionStage::Connecting));
    }

    #[test]
    fn health_state_flips() {
        let state = HealthState::new();
        assert!(!state.is_ready());
        state.mark_ready();
        assert!(state.is_ready());
        state.mark_not_ready();
        assert!(!state.is_ready());
    }
}
