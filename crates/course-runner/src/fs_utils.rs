use std::{fs, path::Path};

use anyhow::{Context, Result};

pub fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove {}", path.display()))?;
    }
    Ok(())
}

pub fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to).with_context(|| format!("failed to create {}", to.display()))?;

    for entry in fs::read_dir(from).with_context(|| format!("failed to read {}", from.display()))? {
        let entry = entry?;
        let source = entry.path();
        let destination = to.join(entry.file_name());

        if source.is_dir() {
            copy_dir(&source, &destination)?;
        } else if source.is_file() {
            fs::copy(&source, &destination).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source.display(),
                    destination.display()
                )
            })?;
        }
    }

    Ok(())
}

pub fn print_command_output(bytes: &[u8]) {
    let text = String::from_utf8_lossy(bytes);
    let text = text.trim();
    if !text.is_empty() {
        println!("{text}");
    }
}
