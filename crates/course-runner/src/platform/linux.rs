use std::{
    env,
    path::{Path, PathBuf},
};

pub fn cli_candidates_from_exec(exec: &str) -> Vec<String> {
    if !cfg!(target_os = "linux") {
        return Vec::new();
    }

    let path = Path::new(exec);
    let Some(parent) = path.parent() else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    for command in command_names(exec) {
        push_if_file(&mut candidates, parent.join("bin").join(command));
        if let Some(grandparent) = parent.parent() {
            push_if_file(&mut candidates, grandparent.join("bin").join(command));
        }
    }

    candidates
}

pub fn known_cli_candidates() -> Vec<String> {
    if !cfg!(target_os = "linux") {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    for command in ["code", "code-insiders", "codium"] {
        push_if_file(&mut candidates, PathBuf::from("/usr/bin").join(command));
        push_if_file(
            &mut candidates,
            PathBuf::from("/usr/local/bin").join(command),
        );
        push_if_file(&mut candidates, PathBuf::from("/snap/bin").join(command));
    }

    for command in [
        "com.visualstudio.code",
        "com.visualstudio.code-insiders",
        "com.vscodium.codium",
    ] {
        push_if_file(
            &mut candidates,
            PathBuf::from("/var/lib/flatpak/exports/bin").join(command),
        );
        if let Some(home) = env::var_os("HOME") {
            push_if_file(
                &mut candidates,
                PathBuf::from(home)
                    .join(".local/share/flatpak/exports/bin")
                    .join(command),
            );
        }
    }

    candidates
}

fn command_names(exec: &str) -> &'static [&'static str] {
    if exec.contains("code-insiders") {
        &["code-insiders"]
    } else if exec.contains("codium") || exec.contains("VSCodium") {
        &["codium"]
    } else {
        &["code"]
    }
}

fn push_if_file(candidates: &mut Vec<String>, path: PathBuf) {
    if path.is_file() {
        candidates.push(path.to_string_lossy().into_owned());
    }
}
