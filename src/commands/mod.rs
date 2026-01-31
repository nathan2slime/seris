use crate::types::Data;
use poise::Command;
use std::{error::Error, marker::Send};

pub mod anime;
pub mod clear;
pub mod manga;
pub mod nasa;
pub mod ping;

pub fn commands() -> Vec<Command<Data, Box<dyn Error + Send + Sync + 'static>>> {
    vec![
        ping::ping(),
        clear::clear(),
        nasa::apod(),
        anime::random(),
        manga::random(),
    ]
}
