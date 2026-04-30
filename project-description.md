# project description

This repo is a development-version Solana Rust client course for VS Code. Students use the local course sidebar to read each lesson, reveal hints or the solution, edit Rust TODOs, run checks, track progress, and reset the exercises back to the starter state.

The course is focused on Rust client-side work: building instructions, talking to local RPC, signing transactions, and understanding the small pieces a Solana client uses before a program call or token workflow is sent.

## what is included

- `exercises/*`: learner-facing Rust crates. Each exercise starts with TODOs and tests that define the expected behavior.
- `.course/baseline/exercises`: the starter exercise files used by the reset button.
- `book/src`: lesson text shown in the course sidebar.
- `course.toml`: the course manifest. It maps exercise IDs to lesson files, Cargo packages, hints, solutions, and Surfpool requirements.
- `crates/course-runner`: the Rust CLI used by the sidebar and the terminal. It lists exercises, runs checks, resets files, and starts Surfpool when needed.
- `vscode-extension`: the local VS Code extension that renders the course panel with lesson text, buttons, hints, solution toggle, progress, and reset.
- `.vscode`: VS Code tasks, launch config for extension development, and recommended extensions.

## how it runs

Students start the UI from the VS Code task `Course: Start Course UI`. That task builds the Rust runner, packages the local VS Code extension as a VSIX, and asks the current VS Code executable to install or update it. It does not install system packages, run shell scripts, or delete existing VS Code extensions.

The `Check` button calls:

```bash
cargo run -p course-runner -- check <exercise-id>
```

For exercises that need local Solana state, the runner starts Surfpool in quiet CI mode:

```bash
surfpool start --ci --no-deploy --port 8899 --ws-port 8900 --offline
```

The runner waits for RPC health, sets the local RPC environment for the test process, runs the matching Cargo test, and stops Surfpool afterward. Exercises that only build instructions or derive addresses do not start Surfpool.

Progress is stored in VS Code workspace storage. Reset asks for confirmation, restores `exercises/*` from `.course/baseline/exercises`, and clears the saved progress state.

## what students practice

- Creating Solana keypairs and reading account addresses.
- Requesting airdrops and checking balances.
- Sending SOL with a signed System Program transfer.
- Building Token-2022 metadata and mint-to instruction flows.
- Deriving and creating Associated Token Accounts.
- Building checked token transfers and close-account instructions.
- Creating a Metaplex Core NFT-style asset instruction.
- Building RPC clients and reading node/account state.
- Loading Solana CLI-style keypair files.
- Confirming transaction signatures before reading post-transaction state.
- Creating rent-exempt system accounts.
- Deriving PDAs with stable seed ordering and bump handling.
- Simulating transactions before sending.

## dependency notes

The Solana and SPL crates use a compatible stable client cohort instead of always selecting the highest crate number. Some crates have independent major versions: for example, `solana-address 2.x`, `spl-token-interface 2.x`, `spl-token-2022-interface 2.x`, and `spl-associated-token-account-interface 2.x` are current for the APIs used here.

The course stays on the `solana-client 3.x` line until the 4.x client line is stable and the related transaction, signature, and interface crates can move together. `solana-signature` is pinned to the version used by `solana-keypair` and `solana-client` to avoid duplicate `Signature` types.
