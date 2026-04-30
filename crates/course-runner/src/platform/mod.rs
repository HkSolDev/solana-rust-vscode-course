mod linux;
mod macos;
mod windows;
mod wsl;

use std::{env, path::Path, process::Command as StdCommand};

use anyhow::{Context, Result};

pub fn sanitize_vscode_exec(value: Option<String>) -> Option<String> {
    value.filter(|candidate| !candidate.is_empty() && !candidate.contains("${execPath}"))
}

pub fn vscode_command_candidates(vscode_exec: Option<&str>) -> Vec<String> {
    let mut candidates = Vec::new();

    if let Some(exec) = vscode_exec {
        for candidate in macos::cli_candidates_from_exec(exec) {
            push_candidate(&mut candidates, candidate);
        }
        for candidate in linux::cli_candidates_from_exec(exec) {
            push_candidate(&mut candidates, candidate);
        }

        if should_try_raw_vscode_exec(exec) {
            push_candidate(&mut candidates, exec.to_string());
        }
    }

    if cfg!(target_os = "macos") {
        for candidate in macos::known_cli_candidates() {
            push_candidate(&mut candidates, candidate);
        }
    }

    if cfg!(windows) {
        for candidate in windows::known_cli_candidates() {
            push_candidate(&mut candidates, candidate.to_string_lossy().into_owned());
        }
    }

    if cfg!(target_os = "linux") {
        for candidate in linux::known_cli_candidates() {
            push_candidate(&mut candidates, candidate);
        }
    }

    for command in ["code", "code-insiders", "codium"] {
        push_candidate(&mut candidates, command.to_string());
    }

    candidates
}

pub fn open_course_window(
    root: &Path,
    vscode_exec: Option<&str>,
    vscode_commands: &[String],
    workspace: &Path,
    dry_run: bool,
) -> Result<()> {
    if dry_run {
        println!(
            "Would open VS Code course window for {}",
            workspace.display()
        );
        for command in vscode_commands {
            println!("Would try VS Code command `{command}`.");
        }
        return Ok(());
    }

    if wsl::is_wsl() {
        wsl::open_course_window(vscode_exec, vscode_commands, workspace);
        return Ok(());
    }

    for command in vscode_commands {
        if open_with_vscode_command(command, workspace) {
            return Ok(());
        }
    }

    if macos::open_with_app(workspace) {
        return Ok(());
    }

    println!(
        "Installed the UI, but could not open a fresh VS Code window automatically. Reopen {} in VS Code.",
        root.display()
    );
    Ok(())
}

pub fn home_dir() -> Result<std::path::PathBuf> {
    let key = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    env::var_os(key)
        .map(std::path::PathBuf::from)
        .with_context(|| format!("could not determine home directory from {key}"))
}

pub fn is_wsl() -> bool {
    wsl::is_wsl()
}

fn push_candidate(candidates: &mut Vec<String>, candidate: String) {
    if !candidate.is_empty() && !candidates.iter().any(|existing| existing == &candidate) {
        candidates.push(candidate);
    }
}

fn should_try_raw_vscode_exec(exec: &str) -> bool {
    if cfg!(target_os = "macos") && exec.contains(".app/Contents/MacOS/") {
        return false;
    }

    true
}

fn open_with_vscode_command(command: &str, workspace: &Path) -> bool {
    match StdCommand::new(command)
        .arg("--new-window")
        .arg(workspace)
        .status()
    {
        Ok(status) if status.success() => {
            println!("Opened fresh course window with VS Code command `{command}`.");
            true
        }
        Ok(status) => {
            println!("VS Code command `{command}` exited with {status} while opening the course.");
            false
        }
        Err(error) => {
            println!("Could not run VS Code command `{command}`: {error}");
            false
        }
    }
}
