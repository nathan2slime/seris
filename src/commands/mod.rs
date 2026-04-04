//! Slash command registrations.

use crate::types::{Data, Error};
use poise::Command;

pub mod anime;
pub mod clear;
pub mod manga;
pub mod nasa;
pub mod ping;

/// Returns every registered Discord slash command.
pub fn commands() -> Vec<Command<Data, Error>> {
    vec![
        ping::ping(),
        clear::clear(),
        nasa::apod(),
        anime::random(),
        manga::random(),
    ]
}

#[cfg(test)]
mod tests {
    use super::commands;

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
                "get_random_manga"
            ]
        );
    }
}
