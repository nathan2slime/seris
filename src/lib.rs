#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "bot")]
pub mod commands;
#[cfg(feature = "bot")]
pub mod config;
#[cfg(feature = "bot")]
pub mod embeds;
#[cfg(feature = "bot")]
pub mod reminders;
#[cfg(feature = "bot")]
pub mod services;
#[cfg(test)]
mod test_utils;
#[cfg(feature = "bot")]
pub mod types;
#[cfg(feature = "bot")]
pub mod utils;
