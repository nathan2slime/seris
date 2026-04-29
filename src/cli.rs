/// CLI subcommands supported by the Seris binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    RunBot,
    SelfUpdate,
    Help,
    Unknown(String),
}

/// Parses the first positional CLI argument.
pub fn parse<I>(mut args: I) -> Command
where
    I: Iterator<Item = String>,
{
    match args.next().as_deref() {
        None => Command::RunBot,
        Some("self-update") => Command::SelfUpdate,
        Some("-h") | Some("--help") | Some("help") => Command::Help,
        Some(other) => Command::Unknown(other.to_string()),
    }
}

/// Returns the help text for the binary.
pub fn usage() -> &'static str {
    "Usage: seris [self-update]\n\nCommands:\n  self-update   Download and install the latest release binary\n"
}

#[cfg(test)]
mod tests {
    use super::{parse, Command};

    #[test]
    fn parses_known_commands() {
        assert_eq!(
            parse(["self-update".to_string()].into_iter()),
            Command::SelfUpdate
        );
        assert_eq!(parse(["--help".to_string()].into_iter()), Command::Help);
        assert_eq!(parse(["help".to_string()].into_iter()), Command::Help);
    }

    #[test]
    fn defaults_to_running_the_bot() {
        assert_eq!(parse(std::iter::empty()), Command::RunBot);
    }

    #[test]
    fn captures_unknown_command() {
        assert_eq!(
            parse(["something-else".to_string()].into_iter()),
            Command::Unknown("something-else".to_string())
        );
    }
}
