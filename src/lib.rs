#[cfg(feature = "bot")]
pub mod benchmarks;
pub mod cli;
#[cfg(feature = "bot")]
pub mod commands;
#[cfg(feature = "bot")]
pub mod config;
#[cfg(feature = "bot")]
pub mod database;
#[cfg(feature = "bot")]
pub mod embeds;
#[cfg(feature = "bot")]
pub mod epic;
#[cfg(feature = "bot")]
pub mod plugins;
#[cfg(feature = "bot")]
pub mod services;
#[cfg(test)]
mod test_utils;
#[cfg(feature = "bot")]
pub mod types;
#[cfg(feature = "bot")]
pub mod update;
#[cfg(feature = "bot")]
pub mod utils;
