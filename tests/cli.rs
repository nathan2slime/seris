use seris::cli::{dispatch_from, CliAction};

#[test]
fn no_args_runs_bot() {
    assert!(matches!(
        dispatch_from(Vec::<&str>::new()),
        Ok(CliAction::RunBot)
    ));
}

#[test]
fn version_exits_successfully() {
    assert!(matches!(dispatch_from(["version"]), Ok(CliAction::Exit(0))));
}

#[test]
fn unknown_command_errors() {
    assert!(dispatch_from(["unknown"]).is_err());
}
