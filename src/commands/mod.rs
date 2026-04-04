use crate::types::{Data, Error};
use poise::Command;

pub mod anime;
pub mod clear;
pub mod manga;
pub mod nasa;
pub mod ping;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![
        ping::ping(),
        clear::clear(),
        nasa::apod(),
        anime::random(),
        manga::random(),
    ]
}
