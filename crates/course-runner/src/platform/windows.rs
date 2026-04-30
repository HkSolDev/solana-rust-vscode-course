use std::{env, path::PathBuf};

pub fn known_cli_candidates() -> Vec<PathBuf> {
    let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
        return Vec::new();
    };
    let local_app_data = PathBuf::from(local_app_data);
    [
        local_app_data.join("Programs/Microsoft VS Code/Code.exe"),
        local_app_data.join("Programs/Microsoft VS Code Insiders/Code - Insiders.exe"),
        local_app_data.join("Programs/VSCodium/VSCodium.exe"),
    ]
    .into_iter()
    .filter(|candidate| candidate.is_file())
    .collect()
}

pub fn to_windows_executable_path(value: &str) -> Option<String> {
    if value.ends_with(".exe") && value.as_bytes().get(1) == Some(&b':') {
        return Some(value.to_string());
    }

    let mut parts = value.split('/');
    if parts.next() == Some("") && parts.next() == Some("mnt") {
        if let Some(drive) = parts.next() {
            if drive.len() == 1 {
                let rest = parts.collect::<Vec<_>>().join("\\");
                return Some(format!("{}:\\{}", drive.to_uppercase(), rest));
            }
        }
    }

    None
}
