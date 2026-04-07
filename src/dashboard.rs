//! Lightweight admin dashboard and runtime status server.

use std::{
    fmt::Write as _,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use log::{info, warn};
use serenity::gateway::ConnectionStage;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::types::Error;

const DASHBOARD_ADDR: &str = "0.0.0.0:8080";

/// Shared runtime state exposed to the dashboard.
pub struct DashboardState {
    ready: AtomicBool,
    started_at: Instant,
}

impl DashboardState {
    /// Creates a new dashboard state snapshot.
    pub fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            started_at: Instant::now(),
        }
    }

    /// Marks the bot as ready.
    pub fn mark_ready(&self) {
        self.ready.store(true, Ordering::SeqCst);
    }

    /// Marks the bot as not ready.
    pub fn mark_not_ready(&self) {
        self.ready.store(false, Ordering::SeqCst);
    }

    /// Returns whether the bot is connected and ready.
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }

    /// Returns the elapsed runtime since startup.
    pub fn uptime(&self) -> Duration {
        self.started_at.elapsed()
    }
}

impl Default for DashboardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns whether the shard stage indicates a connected session.
pub fn stage_is_ready(stage: &ConnectionStage) -> bool {
    matches!(stage, ConnectionStage::Connected)
}

/// Applies a shard stage change to the dashboard state.
pub fn apply_stage_update(state: &DashboardState, stage: ConnectionStage) {
    if stage_is_ready(&stage) {
        state.mark_ready();
    } else {
        state.mark_not_ready();
    }
}

/// Starts the dashboard and status server.
pub async fn start(state: Arc<DashboardState>) -> Result<(), Error> {
    let listener = TcpListener::bind(DASHBOARD_ADDR).await?;
    let addr = listener.local_addr()?;
    info!("dashboard server listening on {addr}");

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

            let response = response_for_path(path, state.is_ready(), state.uptime());
            let body_len = response.body.len();
            let response = format!(
                "HTTP/1.1 {status} {status_text}\r\nContent-Type: {content_type}\r\nContent-Length: {body_len}\r\nConnection: close\r\n\r\n{body}",
                status = response.status,
                status_text = response.status_text,
                content_type = response.content_type,
                body_len = body_len,
                body = response.body,
            );

            if let Err(err) = socket.write_all(response.as_bytes()).await {
                warn!("failed to write dashboard response to {peer}: {err}");
            }
        });
    }
}

struct Response {
    status: u16,
    status_text: &'static str,
    content_type: &'static str,
    body: String,
}

fn response_for_path(path: &str, ready: bool, uptime: Duration) -> Response {
    match path {
        "/" | "/dashboard" => Response {
            status: 200,
            status_text: "OK",
            content_type: "text/html; charset=utf-8",
            body: dashboard_page(ready, uptime),
        },
        "/health" => Response {
            status: 200,
            status_text: "OK",
            content_type: "text/plain; charset=utf-8",
            body: "ok".to_string(),
        },
        "/ready" if ready => Response {
            status: 200,
            status_text: "OK",
            content_type: "text/plain; charset=utf-8",
            body: "ready".to_string(),
        },
        "/ready" => Response {
            status: 503,
            status_text: "Service Unavailable",
            content_type: "text/plain; charset=utf-8",
            body: "not ready".to_string(),
        },
        _ => Response {
            status: 404,
            status_text: "Not Found",
            content_type: "text/plain; charset=utf-8",
            body: "not found".to_string(),
        },
    }
}

fn dashboard_page(ready: bool, uptime: Duration) -> String {
    let status_label = if ready { "Ready" } else { "Connecting" };
    let status_blurb = if ready {
        "The bot is connected to Discord and can serve commands."
    } else {
        "The bot is starting up or reconnecting to Discord."
    };
    let mut html = String::new();

    write!(
        html,
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>Seris Admin Dashboard</title><style>body{{margin:0;font-family:system-ui,-apple-system,Segoe UI,sans-serif;background:#0f172a;color:#e2e8f0;}}main{{max-width:820px;margin:0 auto;padding:48px 24px;}}.panel{{background:#111827;border:1px solid #1f2937;border-radius:16px;padding:24px;box-shadow:0 10px 30px rgba(0,0,0,.2);}}.badge{{display:inline-flex;align-items:center;padding:6px 12px;border-radius:999px;font-size:12px;font-weight:700;letter-spacing:.08em;text-transform:uppercase;background:{badge_bg};color:{badge_fg};}}h1{{margin:16px 0 8px;font-size:clamp(2rem,4vw,3rem);}}p{{line-height:1.6;color:#cbd5e1;}}dl{{display:grid;grid-template-columns:140px 1fr;gap:12px 16px;margin:24px 0 0;}}dt{{color:#94a3b8;font-weight:600;}}dd{{margin:0;color:#f8fafc;}}.links{{margin-top:24px;display:flex;flex-wrap:wrap;gap:12px;}}a{{color:#93c5fd;text-decoration:none;}}a:hover{{text-decoration:underline;}}footer{{margin-top:20px;color:#64748b;font-size:14px;}}</style></head><body><main><div class=\"panel\"><span class=\"badge\">{status_label}</span><h1>Seris Admin Dashboard</h1><p>{status_blurb}</p><dl><dt>Uptime</dt><dd>{uptime}</dd><dt>Health</dt><dd><a href=\"/health\">/health</a></dd><dt>Readiness</dt><dd><a href=\"/ready\">/ready</a></dd></dl><div class=\"links\"><a href=\"/dashboard\">Refresh dashboard</a></div><footer>Built for lightweight operational checks and manual oversight.</footer></div></main></body></html>",
        badge_bg = if ready { "#14532d" } else { "#78350f" },
        badge_fg = if ready { "#86efac" } else { "#fcd34d" },
        status_label = status_label,
        status_blurb = status_blurb,
        uptime = format_duration(uptime),
    )
    .expect("dashboard template write");

    html
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
    use super::{
        apply_stage_update, format_duration, response_for_path, stage_is_ready, DashboardState,
    };
    use serenity::gateway::ConnectionStage;
    use std::time::Duration;

    #[test]
    fn dashboard_route_returns_html() {
        let response = response_for_path("/", false, Duration::from_secs(65));

        assert_eq!(response.status, 200);
        assert_eq!(response.content_type, "text/html; charset=utf-8");
        assert!(response.body.contains("Seris Admin Dashboard"));
        assert!(response.body.contains("Connecting"));
        assert!(response.body.contains("1m 05s"));
    }

    #[test]
    fn ready_route_reflects_readiness() {
        let ready = response_for_path("/ready", true, Duration::from_secs(0));
        let not_ready = response_for_path("/ready", false, Duration::from_secs(0));

        assert_eq!(ready.status, 200);
        assert_eq!(ready.body, "ready");
        assert_eq!(not_ready.status, 503);
        assert_eq!(not_ready.body, "not ready");
    }

    #[test]
    fn only_connected_stage_is_ready() {
        assert!(stage_is_ready(&ConnectionStage::Connected));
        assert!(!stage_is_ready(&ConnectionStage::Disconnected));
        assert!(!stage_is_ready(&ConnectionStage::Connecting));
    }

    #[test]
    fn stage_updates_flip_readiness() {
        let state = DashboardState::new();
        apply_stage_update(&state, ConnectionStage::Disconnected);
        assert!(!state.is_ready());

        apply_stage_update(&state, ConnectionStage::Connected);
        assert!(state.is_ready());
    }

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
