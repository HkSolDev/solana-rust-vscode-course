use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseManifest {
    pub course: CourseInfo,
    #[serde(default)]
    pub exercise: Vec<Exercise>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseInfo {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub package: String,
    #[serde(default)]
    pub needs_surfpool: bool,
    #[serde(default)]
    pub surfpool_mode: Option<String>,
    pub lesson: String,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub solution: Option<String>,
}

pub fn find_course_root() -> Result<PathBuf> {
    let mut current = env::current_dir().context("failed to read current directory")?;

    loop {
        if current.join("course.toml").is_file() {
            return Ok(current);
        }

        if !current.pop() {
            bail!("could not find course.toml in this directory or its parents");
        }
    }
}

pub fn load_manifest(root: &Path) -> Result<CourseManifest> {
    let manifest_path = root.join("course.toml");
    let manifest = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;

    toml::from_str(&manifest).context("failed to parse course.toml")
}

pub fn find_exercise<'a>(manifest: &'a CourseManifest, id: &str) -> Result<&'a Exercise> {
    manifest
        .exercise
        .iter()
        .find(|candidate| candidate.id == id)
        .with_context(|| format!("unknown exercise `{id}`"))
}

pub fn list_exercises(manifest: &CourseManifest) {
    for exercise in &manifest.exercise {
        let backend = if exercise.needs_surfpool {
            "surfpool"
        } else {
            "cargo"
        };

        println!("{} - {} ({backend})", exercise.id, exercise.title);
    }
}

pub fn describe_course(manifest: &CourseManifest) {
    println!("{}", manifest.course.title);
    println!("{}", manifest.course.description);
    println!();
    list_exercises(manifest);
}

pub fn print_metadata(manifest: &CourseManifest) -> Result<()> {
    let json = serde_json::to_string_pretty(manifest).context("failed to serialize metadata")?;
    println!("{json}");
    Ok(())
}

pub fn print_hint(exercise: &Exercise, number: usize) -> Result<()> {
    if number == 0 || number > exercise.hints.len() {
        bail!(
            "{} has {} hints; ask for a number from 1 to {}",
            exercise.id,
            exercise.hints.len(),
            exercise.hints.len()
        );
    }

    println!("{}", exercise.hints[number - 1]);
    Ok(())
}

pub fn print_solution(exercise: &Exercise) -> Result<()> {
    let solution = exercise
        .solution
        .as_deref()
        .with_context(|| format!("{} does not have a solution yet", exercise.id))?;

    println!("{solution}");
    Ok(())
}
