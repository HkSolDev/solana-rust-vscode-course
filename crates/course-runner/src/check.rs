use std::path::Path;

use anyhow::{bail, Context, Result};
use tokio::process::Command;

use crate::{manifest::Exercise, surfpool};

pub async fn check_exercise(root: &Path, exercise: &Exercise) -> Result<()> {
    println!("Checking {} - {}", exercise.id, exercise.title);
    println!("Lesson: {}", exercise.lesson);

    let (rpc_port, ws_port) = surfpool::ports();
    let rpc_url = format!("http://127.0.0.1:{rpc_port}");
    let ws_url = format!("ws://127.0.0.1:{ws_port}");

    let mut surfpool = if exercise.needs_surfpool {
        Some(surfpool::start(exercise, &rpc_port, &ws_port).await?)
    } else {
        None
    };

    if exercise.needs_surfpool {
        surfpool::wait_for_rpc(&rpc_url).await?;
    }

    let status = Command::new("cargo")
        .current_dir(root)
        .args(["test", "-p", &exercise.package])
        .env("SOLANA_RPC_URL", &rpc_url)
        .env("SOLANA_WS_URL", &ws_url)
        .status()
        .await
        .context("failed to run cargo test")?;

    if let Some(child) = surfpool.as_mut() {
        let _ = child.start_kill();
        let _ = child.wait().await;
    }

    if !status.success() {
        bail!("{} is not passing yet", exercise.id);
    }

    println!("Passed {}", exercise.id);
    Ok(())
}
