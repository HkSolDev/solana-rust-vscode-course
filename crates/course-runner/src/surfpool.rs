use std::{
    env,
    process::Stdio,
    time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use serde_json::json;
use tokio::process::{Child, Command};
use tokio::time::sleep;

use crate::manifest::Exercise;

pub async fn start(exercise: &Exercise, rpc_port: &str, ws_port: &str) -> Result<Child> {
    let mode = exercise
        .surfpool_mode
        .as_deref()
        .unwrap_or("offline")
        .to_string();

    let mut command = Command::new("surfpool");
    command
        .args([
            "start",
            "--ci",
            "--no-deploy",
            "--port",
            rpc_port,
            "--ws-port",
            ws_port,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true);

    match mode.as_str() {
        "offline" => {
            command.arg("--offline");
        }
        "devnet" | "testnet" | "mainnet" => {
            command.args(["--network", &mode]);
        }
        other if other.starts_with("http://") || other.starts_with("https://") => {
            command.args(["--rpc-url", other]);
        }
        other => bail!("unsupported surfpool_mode `{other}`"),
    }

    println!("Starting Surfpool in {mode} mode on port {rpc_port}...");
    command
        .spawn()
        .context("failed to start `surfpool`; is the Surfpool CLI installed?")
}

pub fn ports() -> (String, String) {
    let rpc_port = env::var("COURSE_RPC_PORT").unwrap_or_else(|_| "8899".to_string());
    let ws_port = env::var("COURSE_WS_PORT").unwrap_or_else(|_| "8900".to_string());
    (rpc_port, ws_port)
}

pub async fn wait_for_rpc(rpc_url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let timeout = Duration::from_secs(20);
    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {
        let response = client
            .post(rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getHealth"
            }))
            .send()
            .await;

        if let Ok(response) = response {
            if response.status().is_success() {
                let body: serde_json::Value = response.json().await.unwrap_or_default();
                if body.get("result").and_then(|value| value.as_str()) == Some("ok") {
                    return Ok(());
                }
            }
        }

        sleep(Duration::from_millis(250)).await;
    }

    bail!("RPC at {rpc_url} did not become healthy within {timeout:?}");
}
