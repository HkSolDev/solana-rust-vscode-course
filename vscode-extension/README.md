# Solana Rust Course UI

This local extension adds the interactive course panel used by the VS Code course.

For learners, start it from the repository root with one VS Code task:

```text
Cmd/Ctrl+Shift+P -> Tasks: Run Task -> Course: Start Course UI
```

That task packages the local extension and installs it into the active VS Code environment. It does not open a fresh window by default. After the task finishes, click the Solana Course activity bar icon in the same window and choose `Open Course Panel`.

If the icon does not appear after the first install, run the task `Course: Fallback Activation (Open New Window)`. That fallback is only for activation trouble; normal use should stay in the current VS Code window.

The panel shows rendered lesson text, check and next buttons, three hints, a solution dropdown, progress percentage, clickable exercise dots, and a reset button. Progress is stored in VS Code workspace storage. Reset restores `exercises` from `.course/baseline/exercises` and asks before continuing when open exercise files have unsaved edits.

During extension development, use the launch configuration:

```text
F5 -> Launch Course UI Extension
```

A new Extension Development Host window opens. The panel should open automatically there too. The Solana Course activity bar item also hosts the sidebar version.

The panel reads course metadata from the Rust CLI:

```bash
cargo run -p course-runner -- metadata
```

Checks are also delegated to the Rust CLI:

```bash
cargo run -p course-runner -- check 01_keypairs
```
