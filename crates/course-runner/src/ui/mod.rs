mod install;
mod vsix;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::platform;

use install::{install_by_copy, install_with_vscode, should_install_into_extension_folder};
use vsix::build_vsix;

#[derive(Debug, Deserialize)]
pub(crate) struct ExtensionPackage {
    pub(crate) name: String,
    pub(crate) publisher: String,
    pub(crate) version: String,
    #[serde(rename = "displayName")]
    pub(crate) display_name: Option<String>,
    pub(crate) description: Option<String>,
}

pub fn start_course_ui(
    root: &Path,
    vscode_exec: Option<String>,
    workspace: Option<PathBuf>,
    dry_run: bool,
    no_open: bool,
) -> Result<()> {
    let workspace = workspace.unwrap_or_else(|| root.to_path_buf());
    let vscode_exec = platform::sanitize_vscode_exec(vscode_exec);
    let vscode_commands = platform::vscode_command_candidates(vscode_exec.as_deref());
    let extension = load_extension_package(root)?;
    let vsix_path = build_vsix(root, &extension, dry_run)?;

    let installed = if should_install_into_extension_folder() {
        install_by_copy(root, &extension, dry_run)?
    } else if !vscode_commands.is_empty() {
        install_with_vscode(&vscode_commands, &vsix_path, dry_run)
            .context("failed to ask VS Code to install the course UI extension")?
    } else {
        false
    };

    if !installed {
        install_by_copy(root, &extension, dry_run)?;
    }

    if no_open {
        println!("Skipping fallback fresh VS Code window because --no-open was passed.");
    } else {
        platform::open_course_window(
            root,
            vscode_exec.as_deref(),
            &vscode_commands,
            &workspace,
            dry_run,
        )?;
    }

    println!();
    println!("Course UI bootstrap finished.");
    if no_open {
        println!("No new VS Code window was opened.");
        println!("Click the Solana Course icon in this window, then choose Open Course Panel.");
        println!(
            "If the icon does not appear, run the VS Code task `Course: Fallback Activation (Open New Window)`."
        );
        println!(
            "If lesson links still show as raw Markdown, the current window has an older extension loaded; run the fallback activation task or reopen this workspace."
        );
    } else {
        println!("Fallback activation requested.");
        println!(
            "A fresh VS Code course window should open. The course panel opens automatically there."
        );
    }
    Ok(())
}

fn load_extension_package(root: &Path) -> Result<ExtensionPackage> {
    let package_path = root.join("vscode-extension/package.json");
    let package = fs::read_to_string(&package_path)
        .with_context(|| format!("failed to read {}", package_path.display()))?;

    serde_json::from_str(&package).context("failed to parse vscode-extension/package.json")
}
