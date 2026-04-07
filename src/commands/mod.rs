//! Slash command registrations.

use crate::plugins;
use crate::types::{Data, Error};
use poise::Command;

pub mod anime;
pub mod clear;
pub mod manga;
pub mod nasa;
pub mod ping;
pub mod utility;

/// Returns every registered Discord slash command.
pub fn commands() -> Vec<Command<Data, Error>> {
    plugins::registry().commands()
}

#[cfg(test)]
mod tests {
    use super::commands;
    use crate::plugins;

    #[test]
    fn registers_all_expected_commands() {
        let names: Vec<_> = commands().into_iter().map(|command| command.name).collect();

        assert_eq!(
            names,
            vec![
                "ping",
                "clear",
                "apod",
                "get_random_anime",
                "get_random_manga",
                "about",
                "uptime"
            ]
        );
    }

    #[test]
    fn registry_groups_commands_by_plugin() {
        assert_eq!(
            plugins::registry().names(),
            vec!["core", "content", "utility"]
        );
    }
}
