use std::path::Path;

use anyhow::{bail, Result};

use crate::fs_utils::{copy_dir, remove_dir_if_exists};

pub fn reset_course(root: &Path, yes: bool) -> Result<()> {
    if !yes {
        bail!("reset rewrites exercise files; rerun with --yes to confirm");
    }

    let baseline = root.join(".course/baseline/exercises");
    let exercises = root.join("exercises");

    if !baseline.is_dir() {
        bail!("could not find reset baseline at {}", baseline.display());
    }

    remove_dir_if_exists(&exercises)?;
    copy_dir(&baseline, &exercises)?;
    println!("Reset exercise files from {}", baseline.display());
    println!("Course progress is stored in VS Code and will be cleared by the course UI.");
    Ok(())
}
