use dashmap::DashMap;
use env_logger::Env;
use log::{error, info, warn};
use seris::cli::{parse, Command as CliCommand};
#[cfg(feature = "bot")]
use seris::commands::commands;
#[cfg(feature = "bot")]
use seris::config::load_config;
#[cfg(feature = "bot")]
use seris::database::Database;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[cfg(feature = "bot")]
use seris::types::{Data, Error};

#[cfg(feature = "bot")]
use serenity::all::{ClientBuilder, GatewayIntents, ShardManager};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
    match parse(std::env::args().skip(1)) {
        CliCommand::SelfUpdate => {
            #[cfg(feature = "bot")]
            {
                if let Err(err) = seris::update::run_self_update().await {
                    error!("self-update failed: {err}");
                    log::logger().flush();
                    std::process::exit(1);
                }

                log::logger().flush();
                return;
            }

            #[cfg(not(feature = "bot"))]
            {
                error!("self-update is unavailable in this build");
                log::logger().flush();
                std::process::exit(1);
            }
        }
        CliCommand::Help => {
            println!("{}", seris::cli::usage());
            return;
        }
        CliCommand::Unknown(command) => {
            eprintln!("Unknown command: {command}\n\n{}", seris::cli::usage());
            std::process::exit(2);
        }
        CliCommand::RunBot => {}
    }

    #[cfg(feature = "bot")]
    {
        if let Err(err) = run().await {
            error!("bot exited with error: {err}");
            log::logger().flush();
            std::process::exit(1);
        }
    }

    #[cfg(not(feature = "bot"))]
    {
        warn!("bot feature is disabled");
        log::logger().flush();
    }
}

#[cfg(feature = "bot")]
async fn run() -> Result<(), Error> {
    info!("starting Seris");
    let config = load_config()?;
    info!("configuration loaded");
    let intents = GatewayIntents::non_privileged();
    let discord_token = config.discord_token.clone();
    let database = Arc::new(Database::open_default()?);
    info!("database ready");
    let started_at = Instant::now();
    let epic_sessions = Arc::new(DashMap::new());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            event_handler: seris::epic::framework_event_handler,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config,
                    database: Arc::clone(&database),
                    epic_sessions: Arc::clone(&epic_sessions),
                    started_at,
                })
            })
        })
        .build();

    let mut client = ClientBuilder::new(discord_token, intents)
        .event_handler(seris::utils::Handler::new())
        .framework(framework)
        .await?;

    info!("discord client configured; connecting");
    let shard_manager = client.shard_manager.clone();

    let run_result = tokio::select! {
        result = client.start() => {
            result?;
            Ok(())
        }
        _ = tokio::signal::ctrl_c() => {
            info!("shutdown requested via Ctrl-C");
            graceful_shutdown(shard_manager).await;
            Ok(())
        }
    };

    log::logger().flush();

    run_result
}

#[cfg(feature = "bot")]
async fn graceful_shutdown(shard_manager: Arc<ShardManager>) {
    match tokio::time::timeout(Duration::from_secs(5), shard_manager.shutdown_all()).await {
        Ok(()) => info!("discord client shut down cleanly"),
        Err(_) => warn!("timed out waiting for discord shutdown"),
    }
}
