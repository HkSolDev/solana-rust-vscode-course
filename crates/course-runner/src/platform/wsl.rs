use std::{env, fs, path::Path, process::Command as StdCommand};

use super::windows;

pub fn is_wsl() -> bool {
    if env::var_os("COURSE_FORCE_WSL").is_some()
        || env::var_os("WSL_DISTRO_NAME").is_some()
        || env::var_os("WSL_INTEROP").is_some()
    {
        return true;
    }

    fs::read_to_string("/proc/version")
        .map(|version| version.to_lowercase().contains("microsoft"))
        .unwrap_or(false)
}

pub fn open_course_window(vscode_exec: Option<&str>, vscode_commands: &[String], workspace: &Path) {
    let distro = env::var("WSL_DISTRO_NAME").unwrap_or_else(|_| "Ubuntu".to_string());
    let folder_uri = format!(
        "vscode-remote://wsl+{}{}",
        uri_encode(&distro),
        uri_encode_path(&workspace.to_string_lossy())
    );
    let windows_exec = vscode_exec
        .and_then(windows::to_windows_executable_path)
        .or_else(|| {
            vscode_commands
                .iter()
                .find_map(|candidate| windows::to_windows_executable_path(candidate))
        });
    let command = if let Some(exec) = windows_exec {
        format!("start \"\" \"{exec}\" --folder-uri \"{folder_uri}\"")
    } else {
        format!("start \"\" \"{folder_uri}\"")
    };

    let opened = StdCommand::new("cmd.exe")
        .args(["/d", "/s", "/c", &command])
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if opened {
        println!("Asked Windows VS Code to open WSL course folder.");
    } else {
        println!("Installed the UI, but could not ask Windows VS Code to open the WSL folder automatically.");
    }
}

fn uri_encode_path(value: &str) -> String {
    value
        .split('/')
        .map(uri_encode)
        .collect::<Vec<_>>()
        .join("/")
}

fn uri_encode(value: &str) -> String {
    let mut output = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            output.push(char::from(byte));
        } else {
            output.push_str(&format!("%{byte:02X}"));
        }
    }
    output
}
