//! Command-line entry points for service and maintenance tasks.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const APP_NAME: &str = "seris";
const REPO_OWNER: &str = "nathan2slime";
const REPO_NAME: &str = "seris";
const SERVICE_NAME: &str = "seris.service";
const SYSTEM_CONFIG_PATH: &str = "/opt/seris/.config/seris/config.toml";

/// Top-level CLI actions.
pub enum CliAction {
    /// Start the Discord bot.
    RunBot,
    /// Exit the process with the provided status code.
    Exit(i32),
}

/// Parses CLI arguments and dispatches to the requested action.
pub fn dispatch() -> Result<CliAction, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(CliAction::RunBot);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => {
            print_help();
            Ok(CliAction::Exit(0))
        }
        "version" | "--version" | "-V" => {
            println!("{} {}", APP_NAME, env!("CARGO_PKG_VERSION"));
            Ok(CliAction::Exit(0))
        }
        "config" => handle_config(&args[1..]),
        "service" => handle_service(&args[1..]),
        "self-update" => handle_self_update(&args[1..]),
        other => Err(format!("unknown command: {other}\n\n{}", usage())),
    }
}

fn handle_config(args: &[String]) -> Result<CliAction, String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(format!("missing config subcommand\n\n{}", usage()));
    };

    match command {
        "path" => {
            println!("{}", config_path().display());
            Ok(CliAction::Exit(0))
        }
        "edit" => {
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            run_sudoedit(config_path(), editor)?;
            Ok(CliAction::Exit(0))
        }
        _ => Err(format!(
            "unknown config subcommand: {command}\n\n{}",
            usage()
        )),
    }
}

fn handle_service(args: &[String]) -> Result<CliAction, String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(format!("missing service subcommand\n\n{}", usage()));
    };

    match command {
        "start" | "stop" | "restart" | "status" => {
            run_systemctl(command)?;
            Ok(CliAction::Exit(0))
        }
        "logs" => {
            let follow = args.iter().any(|arg| arg == "--follow" || arg == "-f");
            run_journalctl_logs(follow)?;
            Ok(CliAction::Exit(0))
        }
        _ => Err(format!(
            "unknown service subcommand: {command}\n\n{}",
            usage()
        )),
    }
}

fn handle_self_update(args: &[String]) -> Result<CliAction, String> {
    let version = args
        .first()
        .cloned()
        .unwrap_or_else(|| "latest".to_string());
    let install_url = if version == "latest" {
        format!("https://github.com/{REPO_OWNER}/{REPO_NAME}/releases/latest/download/install.sh")
    } else {
        format!(
            "https://github.com/{REPO_OWNER}/{REPO_NAME}/releases/download/{version}/install.sh"
        )
    };

    let temp_script = env::temp_dir().join(format!("seris-install-{}.sh", std::process::id()));
    run_command(
        "curl",
        vec![
            "-fsSL".to_string(),
            install_url,
            "-o".to_string(),
            temp_script.display().to_string(),
        ],
        &[],
    )?;

    let mut install_args = vec![temp_script.display().to_string()];
    if version != "latest" {
        install_args.push(version);
    }
    let update_result = run_with_optional_sudo("sh", install_args, None);
    let _ = fs::remove_file(&temp_script);
    update_result?;

    Ok(CliAction::Exit(0))
}

fn run_systemctl(action: &str) -> Result<(), String> {
    run_with_optional_sudo(
        "systemctl",
        vec![action.to_string(), SERVICE_NAME.to_string()],
        None,
    )
}

fn run_journalctl_logs(follow: bool) -> Result<(), String> {
    let mut args = vec![
        "-u".to_string(),
        SERVICE_NAME.to_string(),
        "-n".to_string(),
        "100".to_string(),
    ];
    if follow {
        args.push("-f".to_string());
    }

    run_with_optional_sudo("journalctl", args, None)
}

fn run_with_optional_sudo(
    program: &str,
    args: Vec<String>,
    env_override: Option<(&str, String)>,
) -> Result<(), String> {
    let is_root = current_uid() == Some(0);

    if is_root {
        return run_command(program, args, &env_override.into_iter().collect::<Vec<_>>());
    }

    let mut sudo_args = Vec::with_capacity(args.len() + 2);
    sudo_args.push("-p".to_string());
    sudo_args.push("[seris-chan] sudo password: ".to_string());
    sudo_args.push(program.to_string());
    sudo_args.extend(args);
    run_command(
        "sudo",
        sudo_args,
        &env_override.into_iter().collect::<Vec<_>>(),
    )
}

fn run_sudoedit(path: PathBuf, editor: String) -> Result<(), String> {
    let path_arg = path.display().to_string();

    if current_uid() == Some(0) {
        return run_command(&editor, vec![path_arg], &[]);
    }

    run_command(
        "sudo",
        vec![
            "-p".to_string(),
            "[seris-chan] sudo password: ".to_string(),
            "sudoedit".to_string(),
            path.display().to_string(),
        ],
        &[("EDITOR", editor)],
    )
}

fn run_command(
    program: &str,
    args: Vec<String>,
    env_override: &[(&str, String)],
) -> Result<(), String> {
    let mut command = Command::new(program);
    command.args(&args);
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    for (key, value) in env_override {
        command.env(key, value);
    }

    let status = command
        .status()
        .map_err(|err| format!("failed to run {program}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with status {status}"))
    }
}

fn current_uid() -> Option<u32> {
    let output = Command::new("id").arg("-u").output().ok()?;
    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
}

fn config_path() -> PathBuf {
    PathBuf::from(env::var("SERIS_CONFIG_FILE").unwrap_or_else(|_| SYSTEM_CONFIG_PATH.to_string()))
}

fn print_help() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "seris <command>\n\nCommands:\n  version\n  config path\n  config edit\n  service start\n  service stop\n  service restart\n  service status\n  service logs [--follow]\n  self-update [tag]\n\nWithout a command, Seris starts the bot normally."
}
