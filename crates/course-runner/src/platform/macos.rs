use std::{path::Path, process::Command as StdCommand};

pub fn cli_candidates_from_exec(exec: &str) -> Vec<String> {
    if !cfg!(target_os = "macos") {
        return Vec::new();
    }

    let path = Path::new(exec);
    for ancestor in path.ancestors() {
        if ancestor.extension().and_then(|value| value.to_str()) != Some("app") {
            continue;
        }

        let bin = ancestor.join("Contents/Resources/app/bin");
        let mut candidates = Vec::new();
        for name in ["code", "code-insiders", "codium"] {
            let candidate = bin.join(name);
            if candidate.is_file() {
                candidates.push(candidate.to_string_lossy().into_owned());
            }
        }

        if candidates.is_empty() {
            let app_name = ancestor
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let command = if app_name.contains("Insiders") {
                "code-insiders"
            } else if app_name.contains("Codium") {
                "codium"
            } else {
                "code"
            };
            candidates.push(bin.join(command).to_string_lossy().into_owned());
        }

        return candidates;
    }

    Vec::new()
}

pub fn known_cli_candidates() -> Vec<String> {
    [
        "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code",
        "/Applications/Visual Studio Code - Insiders.app/Contents/Resources/app/bin/code-insiders",
        "/Applications/VSCodium.app/Contents/Resources/app/bin/codium",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

pub fn open_with_app(workspace: &Path) -> bool {
    if !cfg!(target_os = "macos") {
        return false;
    }

    for app in [
        "Visual Studio Code",
        "Visual Studio Code - Insiders",
        "VSCodium",
    ] {
        if StdCommand::new("open")
            .args(["-a", app])
            .arg(workspace)
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
        {
            println!("Opened course folder with macOS app `{app}`.");
            return true;
        }
    }

    false
}
