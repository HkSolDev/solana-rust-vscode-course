# Solana Rust Client Course for VS Code

This is a development-version course for teaching Solana basics from the Rust client side. The Rust CLI and test harness are the source of truth, and the local VS Code extension gives students the clickable course panel.

## Requirements

- VS Code
- Rust toolchain with Cargo
- Surfpool CLI installed and available on `PATH`
- Recommended VS Code extensions: `rust-analyzer` and `Even Better TOML`

Check Surfpool with:

```bash
surfpool --version
```

Surfpool installation options are documented in [Surfpool Getting Started](https://docs.surfpool.run/toolchain/getting-started).

## Student Flow

1. Open the repo in VS Code.
2. Trust the workspace when VS Code asks.
3. Press `Cmd+Shift+P` / `Ctrl+Shift+P`, run `Tasks: Run Task`, then choose `Course: Start Course UI`.
4. Click the Solana Course icon in the activity bar and open the course panel.
5. Read the lesson, edit the TODOs in `exercises/*/src/lib.rs`, then click `Check`.

If the Solana Course icon does not appear after the first install, run `Course: Fallback Activation (Open New Window)`. That task uses the same installer, then opens the repo in a fresh VS Code window so the newly installed extension can activate.

The terminal path works too:

```bash
cargo run -p course-runner -- list
cargo run -p course-runner -- check 01_keypairs
```

## What The Setup Task Does

`Course: Start Course UI` runs:

```bash
cargo run -p course-runner -- start-ui --vscode-exec ${execPath} --workspace ${workspaceFolder} --no-open
```

The runner packages `vscode-extension` into a local VSIX under `.course-vsix`, then asks the current VS Code executable to install it with `--install-extension`. It uses VS Code's `${execPath}`, so students do not need the `code` command in their shell `PATH`.

On WSL or VS Code Server-style setups, the runner installs into the VS Code Server extension folder instead. On Linux desktops it also tries common VS Code, Insiders, VSCodium, Snap, and Flatpak command locations.

The setup task does not install system packages, does not run shell scripts, and does not delete existing VS Code extensions. The fallback task is the only setup path that asks VS Code to open a new window.

## What The Check Button Does

The extension calls:

```bash
cargo run -p course-runner -- check <exercise-id>
```

For exercises that need local chain state, the runner starts Surfpool like this:

```bash
surfpool start --ci --no-deploy --port 8899 --ws-port 8900 --offline
```

The current course exercises use Surfpool in offline mode. The runner waits for RPC health at `http://127.0.0.1:8899`, sets `SOLANA_RPC_URL` and `SOLANA_WS_URL` for the test process, runs the matching `cargo test -p <exercise-package>`, then stops the Surfpool child process.

Pure instruction-building and derivation exercises do not start Surfpool.

## Course UI

The panel renders lessons from `book/src/*.md` and keeps the controls next to the lesson text: `Check`, `Next`, three hints, solution toggle, progress, lesson/code open buttons, and reset.

Progress is stored in VS Code workspace storage. Reset restores starter exercise files from `.course/baseline/exercises` and clears progress. Reset is explicit and asks first; it will overwrite work in `exercises`.

## Scope

The course currently covers keypairs, addresses, airdrops, SOL transfers, Token-2022 metadata, minting, ATAs, checked token transfers, closing token accounts, Metaplex Core NFT instruction building, RPC basics, wallet files, account info, signature confirmation, rent-exempt account creation, PDA derivation, and transaction simulation.
