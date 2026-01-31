use crate::config::AppConfig;

pub struct Data {
    pub config: AppConfig,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
