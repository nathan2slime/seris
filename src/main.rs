use seris::cli::CliAction;
use seris::commands::commands;
use seris::config::load_config;
use seris::types::{Data, Error};

use serenity::all::{ClientBuilder, GatewayIntents};

#[tokio::main]
async fn main() {
    env_logger::init();

    match seris::cli::dispatch() {
        Ok(CliAction::RunBot) => {}
        Ok(CliAction::Exit(code)) => {
            std::process::exit(code);
        }
        Err(message) => {
            log::error!("{message}");
            std::process::exit(1);
        }
    }

    if let Err(err) = run().await {
        log::error!("{err}");
        std::process::exit(1);
    }
}

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

    tokio::select! {
        result = client.start() => {
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("received ctrl-c");
        }
    }

    Ok(())
}
