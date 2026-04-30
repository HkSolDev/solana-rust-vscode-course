use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command as StdCommand,
};

use anyhow::{Context, Result};

use crate::{
    fs_utils::{copy_dir, print_command_output},
    platform,
};

use super::ExtensionPackage;

pub(crate) fn install_with_vscode(
    commands: &[String],
    vsix_path: &Path,
    dry_run: bool,
) -> Result<bool> {
    if dry_run {
        for command in commands {
            println!(
                "Would install {} with VS Code command {}",
                vsix_path.display(),
                command
            );
        }
        return Ok(true);
    }

    for command in commands {
        let output = match StdCommand::new(command)
            .args(["--install-extension"])
            .arg(vsix_path)
            .output()
        {
            Ok(output) => output,
            Err(error) => {
                println!("Could not run VS Code command `{command}`: {error}");
                continue;
            }
        };

        if output.status.success() {
            println!("Installed course UI extension through VS Code command `{command}`.");
            print_command_output(&output.stdout);
            print_command_output(&output.stderr);
            return Ok(true);
        }

        println!("VS Code command `{command}` did not install the extension.");
        print_command_output(&output.stdout);
        print_command_output(&output.stderr);
    }

    println!(
        "VS Code extension install did not complete; falling back to extension-folder install."
    );
    Ok(false)
}

pub(crate) fn install_by_copy(
    root: &Path,
    extension: &ExtensionPackage,
    dry_run: bool,
) -> Result<bool> {
    let source = root.join("vscode-extension");
    let destination_root = extension_root()?;
    let destination = destination_root.join(format!(
        "{}.{}-{}",
        extension.publisher, extension.name, extension.version
    ));

    if dry_run {
        println!(
            "Would copy course UI extension into {}",
            destination.display()
        );
        return Ok(true);
    }

    fs::create_dir_all(&destination_root)
        .with_context(|| format!("failed to create {}", destination_root.display()))?;
    copy_dir(&source, &destination)?;
    println!(
        "Installed course UI extension by copying files into {}",
        destination.display()
    );
    Ok(true)
}

pub(crate) fn should_install_into_extension_folder() -> bool {
    env::var_os("COURSE_VSCODE_EXTENSIONS_DIR").is_some()
        || env::var_os("VSCODE_AGENT_FOLDER").is_some()
        || platform::is_wsl()
}

fn extension_root() -> Result<PathBuf> {
    if let Some(value) = env::var_os("VSCODE_PORTABLE") {
        return Ok(PathBuf::from(value).join("extensions"));
    }

    if let Some(value) = env::var_os("COURSE_VSCODE_EXTENSIONS_DIR") {
        return Ok(PathBuf::from(value));
    }

    if let Some(value) = env::var_os("VSCODE_AGENT_FOLDER") {
        return Ok(PathBuf::from(value).join("extensions"));
    }

    let home = platform::home_dir()?;
    if platform::is_wsl() {
        return Ok(home.join(".vscode-server/extensions"));
    }

    Ok(home.join(".vscode/extensions"))
}
