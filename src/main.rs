#[cfg(feature = "bot")]
use seris::commands::commands;
#[cfg(feature = "bot")]
use seris::config::load_config;
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
        .event_handler(seris::utils::Handler)
        .framework(framework)
        .await?;

    let shard_manager = client.shard_manager.clone();

    tokio::select! {
        result = client.start() => {
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("received ctrl-c");
            graceful_shutdown(shard_manager).await;
        }
    }

    log::logger().flush();

    Ok(())
}

#[cfg(feature = "bot")]
async fn graceful_shutdown(shard_manager: Arc<ShardManager>) {
    match tokio::time::timeout(Duration::from_secs(5), shard_manager.shutdown_all()).await {
        Ok(()) => log::info!("discord client shut down cleanly"),
        Err(_) => log::warn!("timed out waiting for discord shutdown"),
    }
}
