#[cfg(feature = "bot")]
use seris::commands::commands;
#[cfg(feature = "bot")]
use seris::config::load_config;
#[cfg(feature = "bot")]
use seris::dashboard::DashboardState;
#[cfg(feature = "bot")]
use seris::health::HealthState;
use std::{sync::Arc, time::Duration};

#[cfg(feature = "bot")]
use seris::types::{Data, Error};

#[cfg(feature = "bot")]
use serenity::all::{ClientBuilder, GatewayIntents, ShardManager};

#[tokio::main]
async fn main() {
    env_logger::init();

    let run_bot = {
        #[cfg(feature = "cli")]
        {
            match seris::cli::dispatch() {
                Ok(seris::cli::CliAction::RunBot) => true,
                Ok(seris::cli::CliAction::Exit(code)) => {
                    log::logger().flush();
                    std::process::exit(code);
                }
                Err(message) => {
                    log::error!("{message}");
                    log::logger().flush();
                    std::process::exit(1);
                }
            }
        }

        #[cfg(not(feature = "cli"))]
        {
            true
        }
    };

    if run_bot {
        #[cfg(feature = "bot")]
        {
            if let Err(err) = run().await {
                log::error!("{err}");
                log::logger().flush();
                std::process::exit(1);
            }
        }

        #[cfg(not(feature = "bot"))]
        {
            log::warn!("bot feature is disabled");
            log::logger().flush();
        }
    }
}

#[cfg(feature = "bot")]
async fn run() -> Result<(), Error> {
    let config = load_config()?;
    let intents = GatewayIntents::non_privileged();
    let discord_token = config.discord_token.clone();
    let dashboard = Arc::new(DashboardState::new());
    let health = Arc::new(HealthState::new());

    let dashboard_task = tokio::task::spawn_blocking({
        let dashboard = Arc::clone(&dashboard);
        move || {
            if let Err(err) = seris::dashboard::start(dashboard) {
                log::error!("dashboard TUI failed: {err}");
            }
        }
    });

    tokio::spawn({
        let health = Arc::clone(&health);
        async move {
            if let Err(err) = seris::health::start(health).await {
                log::error!("health server failed: {err}");
            }
        }
    });

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { config })
            })
        })
        .build();

    let mut client = ClientBuilder::new(discord_token, intents)
        .event_handler(seris::utils::Handler::new(
            Arc::clone(&dashboard),
            Arc::clone(&health),
        ))
        .framework(framework)
        .await?;

    let shard_manager = client.shard_manager.clone();

    let run_result = tokio::select! {
        result = client.start() => {
            result?;
            Ok(())
        }
        _ = tokio::signal::ctrl_c() => {
            dashboard.request_exit();
            graceful_shutdown(shard_manager).await;
            Ok(())
        }
        _ = wait_for_dashboard_exit(Arc::clone(&dashboard)) => {
            log::info!("dashboard requested shutdown");
            graceful_shutdown(shard_manager).await;
            Ok(())
        }
    };

    dashboard.request_exit();
    let _ = dashboard_task.await;
    log::logger().flush();

    run_result
}

#[cfg(feature = "bot")]
async fn graceful_shutdown(shard_manager: Arc<ShardManager>) {
    match tokio::time::timeout(Duration::from_secs(5), shard_manager.shutdown_all()).await {
        Ok(()) => log::info!("discord client shut down cleanly"),
        Err(_) => log::warn!("timed out waiting for discord shutdown"),
    }
}

#[cfg(feature = "bot")]
async fn wait_for_dashboard_exit(state: Arc<DashboardState>) {
    while !state.should_exit() {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}
