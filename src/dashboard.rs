//! Terminal dashboard for the bot runtime.

use std::{
    io::{self, stdout, IsTerminal, Stdout},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};
use serenity::gateway::ConnectionStage;

use crate::types::Error;

const TICK_RATE: Duration = Duration::from_millis(250);

/// Shared runtime state exposed in the terminal dashboard.
pub struct DashboardState {
    ready: AtomicBool,
    stop_requested: AtomicBool,
    started_at: Instant,
}

impl DashboardState {
    /// Creates a new dashboard state snapshot.
    pub fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            stop_requested: AtomicBool::new(false),
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

    /// Requests the dashboard loop to stop.
    pub fn request_exit(&self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }

    /// Returns whether shutdown has been requested.
    pub fn should_exit(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
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

/// Starts the terminal dashboard.
pub fn start(state: Arc<DashboardState>) -> Result<(), Error> {
    if !io::stdout().is_terminal() {
        info!("dashboard TUI disabled because stdout is not a terminal");
        while !state.should_exit() {
            thread::sleep(TICK_RATE);
        }

        return Ok(());
    }

    let mut session = TerminalSession::enter()?;
    session.run(&state)?;
    Ok(())
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalSession {
    fn enter() -> Result<Self, Error> {
        enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    fn run(&mut self, state: &DashboardState) -> Result<(), Error> {
        while !state.should_exit() {
            self.terminal.draw(|frame| render_dashboard(frame, state))?;

            if event::poll(TICK_RATE)? {
                match event::read()? {
                    Event::Key(key)
                        if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat =>
                    {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => state.request_exit(),
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.request_exit();
                            }
                            _ => {}
                        }
                    }
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        );
        let _ = self.terminal.show_cursor();
    }
}

struct DashboardSnapshot {
    ready: bool,
    status_label: &'static str,
    status_message: &'static str,
    uptime: String,
}

fn dashboard_snapshot(state: &DashboardState) -> DashboardSnapshot {
    let ready = state.is_ready();

    DashboardSnapshot {
        ready,
        status_label: if ready { "CONNECTED" } else { "CONNECTING" },
        status_message: if ready {
            "Discord connection is healthy and commands can run."
        } else {
            "Waiting for the Discord gateway to finish connecting."
        },
        uptime: format_duration(state.uptime()),
    }
}

fn render_dashboard(frame: &mut ratatui::Frame<'_>, state: &DashboardState) {
    let snapshot = dashboard_snapshot(state);
    let area = frame.area();

    let outer = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " Seris Admin Dashboard ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" TUI ", Style::default().fg(Color::DarkGray)),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(9),
            Constraint::Min(7),
            Constraint::Length(2),
        ])
        .split(inner);

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "Status: ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                snapshot.status_label,
                Style::default().fg(status_color(snapshot.ready)),
            ),
        ]),
        Line::from(vec![Span::raw(snapshot.status_message)]),
        Line::from(vec![
            Span::styled(
                "Uptime: ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(snapshot.uptime, Style::default().fg(Color::White)),
        ]),
    ])
    .block(Block::default().title("Runtime").borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(chunks[1]);

    let readiness = Gauge::default()
        .block(Block::default().title("Readiness").borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(status_color(snapshot.ready))
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent(if snapshot.ready { 100 } else { 0 })
        .label(snapshot.status_label);
    frame.render_widget(readiness, middle[0]);

    let panel = Paragraph::new(vec![
        Line::from("Live signals"),
        Line::from("- /health and /ready remain available on port 8080"),
        Line::from("- the dashboard follows Discord gateway readiness"),
        Line::from("- q or Ctrl-C closes the TUI"),
    ])
    .block(
        Block::default()
            .title("Operator Notes")
            .borders(Borders::ALL),
    );
    frame.render_widget(panel, middle[1]);

    let commands = List::new(vec![
        ListItem::new("/ping"),
        ListItem::new("/clear"),
        ListItem::new("/nasa apod"),
        ListItem::new("/anime random"),
        ListItem::new("/manga random"),
    ])
    .block(
        Block::default()
            .title("Slash Commands")
            .borders(Borders::ALL),
    );
    frame.render_widget(commands, chunks[2]);

    let footer = Paragraph::new("q / Ctrl-C to exit the dashboard").style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    );
    frame.render_widget(footer, chunks[3]);
}

fn status_color(ready: bool) -> Color {
    if ready {
        Color::Green
    } else {
        Color::Yellow
    }
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
    use super::{dashboard_snapshot, format_duration, stage_is_ready, DashboardState};
    use serenity::gateway::ConnectionStage;
    use std::time::Duration;

    #[test]
    fn only_connected_stage_is_ready() {
        assert!(stage_is_ready(&ConnectionStage::Connected));
        assert!(!stage_is_ready(&ConnectionStage::Disconnected));
        assert!(!stage_is_ready(&ConnectionStage::Connecting));
    }

    #[test]
    fn dashboard_snapshot_reflects_readiness() {
        let state = DashboardState::new();
        let snapshot = dashboard_snapshot(&state);

        assert_eq!(snapshot.status_label, "CONNECTING");
        assert!(snapshot.status_message.contains("Waiting"));

        state.mark_ready();
        let snapshot = dashboard_snapshot(&state);

        assert_eq!(snapshot.status_label, "CONNECTED");
        assert!(snapshot.status_message.contains("healthy"));
    }

    #[test]
    fn dashboard_state_exit_flag_flips() {
        let state = DashboardState::new();

        assert!(!state.should_exit());
        state.request_exit();
        assert!(state.should_exit());
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
