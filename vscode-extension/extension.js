const vscode = require("vscode");
const fs = require("fs");
const path = require("path");
const { execFile } = require("child_process");

function activate(context) {
  const output = vscode.window.createOutputChannel("Solana Rust Course");
  output.appendLine("Activating Solana Rust Course UI.");
  const provider = new CourseViewProvider(context);
  provider.output = output;

  context.subscriptions.push(
    output,
    vscode.window.registerWebviewViewProvider("solanaRustCourse.view", provider),
    vscode.commands.registerCommand("solanaRustCourse.showPanel", () => {
      provider.showPanel();
    })
  );

  if (findCourseRoot(context)) {
    setTimeout(() => {
      provider
        .showPanel()
        .catch((error) => output.appendLine(`Auto-open failed: ${error.message}`));
    }, 600);
  }
}

function deactivate() {}

class CourseViewProvider {
  constructor(context) {
    this.context = context;
    this.view = undefined;
    this.panel = undefined;
    this.output = undefined;
  }

  resolveWebviewView(webviewView) {
    this.view = webviewView;
    webviewView.webview.options = this.webviewOptions();
    webviewView.webview.html = this.renderLoading(webviewView.webview);

    webviewView.webview.onDidReceiveMessage((message) =>
      this.handleWebviewMessage(webviewView.webview, message)
    );

    this.refresh();
  }

  async showPanel() {
    if (this.panel) {
      this.panel.reveal(vscode.ViewColumn.One);
      await this.refresh();
      return;
    }

    this.panel = vscode.window.createWebviewPanel(
      "solanaRustCourse.panel",
      "Solana Rust Course",
      vscode.ViewColumn.One,
      { ...this.webviewOptions(), retainContextWhenHidden: true }
    );

    this.panel.webview.html = this.renderLoading(this.panel.webview);
    this.panel.webview.onDidReceiveMessage((message) =>
      this.handleWebviewMessage(this.panel.webview, message)
    );
    this.panel.onDidDispose(() => {
      this.panel = undefined;
    });

    await this.refresh();
  }

  async handleWebviewMessage(webview, message) {
    if (message.type === "check") {
      await this.checkExercise(message.exerciseId, webview);
      return;
    }

    if (message.type === "markCurrent") {
      await this.context.workspaceState.update("currentExerciseId", message.exerciseId);
      return;
    }

    if (message.type === "openFile") {
      await this.openCourseFile(message.file);
      return;
    }

    if (message.type === "openExternal") {
      await this.openExternal(message.url);
      return;
    }

    if (message.type === "resetCourse") {
      await this.resetCourse(webview);
    }
  }

  async refresh() {
    if (!this.view && !this.panel) {
      return;
    }

    const root = findCourseRoot(this.context);
    if (!root) {
      this.setHtml((webview) =>
        this.renderMessage(webview, "Open the course workspace containing course.toml.")
      );
      return;
    }

    try {
      this.output?.appendLine(`Loading course metadata from ${root}.`);
      const metadata = await runCourseRunner(root, ["metadata"]);
      const course = JSON.parse(metadata.stdout);
      const currentExerciseId =
        this.context.workspaceState.get("currentExerciseId") ||
        course.exercise?.[0]?.id;
      const passed = this.context.workspaceState.get("passedExercises") || {};
      const lessons = loadLessons(root, course);

      this.setHtml((webview) =>
        this.renderCourse(webview, course, currentExerciseId, passed, lessons)
      );
    } catch (error) {
      this.output?.appendLine(`Metadata load failed: ${error.message}`);
      this.setHtml((webview) =>
        this.renderMessage(webview, `Could not load course metadata.\n\n${error.message}`)
      );
    }
  }

  setHtml(render) {
    if (this.view) {
      this.view.webview.html = render(this.view.webview);
    }

    if (this.panel) {
      this.panel.webview.html = render(this.panel.webview);
    }
  }

  async openCourseFile(file) {
    const root = findCourseRoot(this.context);
    if (!root || !file) {
      return;
    }

    const target = vscode.Uri.file(path.join(root, file));
    const document = await vscode.workspace.openTextDocument(target);
    await vscode.window.showTextDocument(document, { preview: false });
  }

  async openExternal(url) {
    if (!isAllowedExternalUrl(url)) {
      return;
    }

    await vscode.env.openExternal(vscode.Uri.parse(url));
  }

  async checkExercise(exerciseId, sourceWebview) {
    if (!this.view && !this.panel) {
      return;
    }

    const root = findCourseRoot(this.context);
    if (!root) {
      return;
    }

    this.postMessage(sourceWebview, {
      type: "checkStarted",
      exerciseId
    });

    try {
      this.output?.appendLine(`Checking ${exerciseId}.`);
      const result = await runCourseRunner(root, ["check", exerciseId]);
      const passed = this.context.workspaceState.get("passedExercises") || {};
      passed[exerciseId] = true;
      await this.context.workspaceState.update("passedExercises", passed);

      this.postMessage(sourceWebview, {
        type: "checkFinished",
        exerciseId,
        ok: true,
        output: result.stdout || "Passed."
      });
    } catch (error) {
      const passed = this.context.workspaceState.get("passedExercises") || {};
      passed[exerciseId] = false;
      await this.context.workspaceState.update("passedExercises", passed);

      this.output?.appendLine(`Check failed for ${exerciseId}: ${error.message}`);
      this.postMessage(sourceWebview, {
        type: "checkFinished",
        exerciseId,
        ok: false,
        output: error.output || error.message
      });
    }
  }

  async resetCourse(sourceWebview) {
    const root = findCourseRoot(this.context);
    if (!root) {
      return;
    }

    const dirtyExerciseDocuments = vscode.workspace.textDocuments.filter(
      (document) =>
        document.isDirty &&
        document.uri.scheme === "file" &&
        isInsidePath(document.uri.fsPath, path.join(root, "exercises"))
    );

    const action = await vscode.window.showWarningMessage(
      dirtyExerciseDocuments.length > 0
        ? `Reset will restore starter exercise files and clear progress. ${dirtyExerciseDocuments.length} open exercise file(s) have unsaved edits. Save them before reset?`
        : "Reset will restore starter exercise files and clear course progress.",
      { modal: true },
      dirtyExerciseDocuments.length > 0 ? "Save And Reset" : "Reset Course",
      "Cancel"
    );

    if (action !== "Reset Course" && action !== "Save And Reset") {
      return;
    }

    if (dirtyExerciseDocuments.length > 0) {
      for (const document of dirtyExerciseDocuments) {
        const saved = await document.save();
        if (!saved) {
          vscode.window.showWarningMessage("Reset cancelled because an exercise file could not be saved.");
          return;
        }
      }
    }

    this.postMessage(sourceWebview, {
      type: "resetStarted"
    });

    try {
      this.output?.appendLine("Resetting course files and progress.");
      const result = await runCourseRunner(root, ["reset", "--yes"]);
      await this.context.workspaceState.update("passedExercises", {});
      await this.context.workspaceState.update("currentExerciseId", undefined);

      vscode.window.showInformationMessage("Course reset to starter files.");
      this.postMessage(sourceWebview, {
        type: "resetFinished",
        ok: true,
        output: result.stdout || "Course reset."
      });
      await this.refresh();
    } catch (error) {
      this.output?.appendLine(`Reset failed: ${error.message}`);
      this.postMessage(sourceWebview, {
        type: "resetFinished",
        ok: false,
        output: error.output || error.message
      });
    }
  }

  postMessage(sourceWebview, message) {
    if (sourceWebview) {
      sourceWebview.postMessage(message);
      return;
    }

    if (this.view) {
      this.view.webview.postMessage(message);
    }

    if (this.panel) {
      this.panel.webview.postMessage(message);
    }
  }

  renderLoading(webview) {
    return this.page(webview, "<main><p>Loading course...</p></main>");
  }

  renderMessage(webview, message) {
    return this.page(webview, `<main><pre>${escapeHtml(message)}</pre></main>`);
  }

  renderCourse(webview, course, currentExerciseId, passed, lessons) {
    const nonce = getNonce();
    const courseJson = escapeScriptJson({ course, currentExerciseId, passed, lessons });
    const iconUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this.context.extensionUri, "media", "icon.svg")
    );
    const body = `
      <main>
        <header class="courseHeader">
          <div class="brandRow">
            <div class="brandTitle">
              <img class="brandMark" src="${escapeHtml(String(iconUri))}" alt="" aria-hidden="true">
              <h1>${escapeHtml(course.course.title)}</h1>
            </div>
          </div>
          <p>${escapeHtml(course.course.description)}</p>
        </header>

        <section id="app"></section>
      </main>

      <script nonce="${nonce}">
        const vscode = acquireVsCodeApi();
        const state = ${courseJson};
        let currentIndex = Math.max(
          0,
          state.course.exercise.findIndex((exercise) => exercise.id === state.currentExerciseId)
        );
        let openHints = {};
        let lastOutput = "";
        let running = false;

        function currentExercise() {
          return state.course.exercise[currentIndex];
        }

        function render() {
          const exercise = currentExercise();
          const isPassed = Boolean(state.passed[exercise.id]);
          const nextExercise = state.course.exercise[currentIndex + 1];
          const hints = exercise.hints || [];
          const shownHints = openHints[exercise.id] || [];
          const lessonHtml = state.lessons[exercise.id] || "<p>Lesson content is not available yet.</p>";
          const totalExercises = state.course.exercise.length;
          const solvedCount = state.course.exercise.filter((candidate) => Boolean(state.passed[candidate.id])).length;
          const progressPercent = totalExercises === 0 ? 0 : Math.round((solvedCount / totalExercises) * 100);
          const app = document.getElementById("app");

          app.innerHTML = \`
            <div class="courseLayout">
              <article class="lessonPane">
                \${lessonHtml}
              </article>

              <section class="controlPane" aria-label="Exercise controls">
                <div class="progressCard">
                  <div class="progressTop">
                    <span>Course Progress</span>
                    <strong>\${progressPercent}%</strong>
                  </div>
                  <div class="progressTrack" aria-hidden="true">
                    <div class="progressFill" style="width: \${progressPercent}%"></div>
                  </div>
                  <div class="progressMeta">\${solvedCount} of \${totalExercises} exercises solved</div>
                  <div class="exerciseDots" aria-label="Exercise progress">
                    \${state.course.exercise
                      .map((candidate, index) => \`
                        <button
                          class="dot \${state.passed[candidate.id] ? "done" : ""} \${index === currentIndex ? "active" : ""}"
                          data-jump="\${index}"
                          aria-label="Open \${escapeHtmlClient(candidate.title)}"
                          title="\${escapeHtmlClient(candidate.title)}"
                        >
                          \${index + 1}
                        </button>
                      \`)
                      .join("")}
                  </div>
                </div>

                <div class="exerciseHeader">
                  <label for="exercise">Exercise</label>
                  <select id="exercise">
                    \${state.course.exercise
                      .map((candidate, index) => \`
                        <option value="\${index}" \${index === currentIndex ? "selected" : ""}>
                          \${escapeHtmlClient(candidate.id)} - \${escapeHtmlClient(candidate.title)}
                        </option>
                      \`)
                      .join("")}
                  </select>
                  <span class="status \${isPassed ? "passed" : ""}">
                    \${isPassed ? "Solved" : "Not solved"}
                  </span>
                </div>

                <div class="actions">
                  <button id="check" class="primaryButton" \${running ? "disabled" : ""}>Check</button>
                  <button id="next" class="secondaryButton" \${isPassed && nextExercise && !running ? "" : "disabled"}>Next</button>
                </div>

                <div class="hintList">
                  \${[0, 1, 2]
                    .map((index) => \`
                      <div class="hintDisclosure \${shownHints.includes(index) ? "open" : ""} \${hints[index] ? "" : "disabled"}">
                        <button
                          class="hintToggle"
                          data-hint="\${index}"
                          aria-expanded="\${shownHints.includes(index) ? "true" : "false"}"
                          \${hints[index] ? "" : "disabled"}
                        >
                          \${hints[index] ? "Hint " + (index + 1) : "Hint " + (index + 1) + " unavailable"}
                        </button>
                        \${shownHints.includes(index) && hints[index]
                          ? \`<p>\${escapeHtmlClient(hints[index])}</p>\`
                          : ""}
                      </div>
                    \`)
                    .join("")}
                  <details class="solution">
                    <summary>Solution</summary>
                    <pre>\${escapeHtmlClient(exercise.solution || "No solution has been added yet.")}</pre>
                  </details>
                </div>

                <div class="links">
                  <button id="openLesson" class="ghostButton">Open lesson source</button>
                  <button id="openExercise" class="ghostButton">Open code</button>
                  <button id="resetCourse" class="dangerButton" \${running ? "disabled" : ""}>Reset course</button>
                </div>

                <pre id="output">\${escapeHtmlClient(lastOutput || "Run Check when you are ready.")}</pre>
              </section>
            </div>
          \`;

          document.getElementById("exercise").addEventListener("change", (event) => {
            currentIndex = Number(event.target.value);
            vscode.postMessage({ type: "markCurrent", exerciseId: currentExercise().id });
            lastOutput = "";
            render();
          });

          document.getElementById("check").addEventListener("click", () => {
            running = true;
            lastOutput = "Checking " + exercise.id + "...";
            render();
            vscode.postMessage({ type: "check", exerciseId: exercise.id });
          });

          document.getElementById("next").addEventListener("click", () => {
            if (isPassed && nextExercise) {
              currentIndex += 1;
              vscode.postMessage({ type: "markCurrent", exerciseId: currentExercise().id });
              lastOutput = "";
              render();
            }
          });

          document.querySelectorAll(".hintToggle").forEach((button) => {
            button.addEventListener("click", () => {
              const hintIndex = Number(button.dataset.hint);
              const list = openHints[exercise.id] || [];
              const position = list.indexOf(hintIndex);
              if (position >= 0) {
                list.splice(position, 1);
              } else {
                list.push(hintIndex);
              }
              openHints[exercise.id] = list;
              render();
            });
          });

          document.querySelectorAll(".dot").forEach((button) => {
            button.addEventListener("click", () => {
              currentIndex = Number(button.dataset.jump);
              vscode.postMessage({ type: "markCurrent", exerciseId: currentExercise().id });
              lastOutput = "";
              render();
            });
          });

          document.getElementById("openLesson").addEventListener("click", () => {
            vscode.postMessage({ type: "openFile", file: exercise.lesson });
          });

          document.getElementById("openExercise").addEventListener("click", () => {
            vscode.postMessage({
              type: "openFile",
              file: "exercises/" + exercise.id + "/src/lib.rs"
            });
          });

          document.getElementById("resetCourse").addEventListener("click", () => {
            vscode.postMessage({ type: "resetCourse" });
          });
        }

        window.addEventListener("message", (event) => {
          const message = event.data;

          if (message.type === "checkStarted") {
            running = true;
            lastOutput = "Checking " + message.exerciseId + "...";
            render();
          }

          if (message.type === "checkFinished") {
            running = false;
            if (message.ok) {
              state.passed[message.exerciseId] = true;
            } else {
              state.passed[message.exerciseId] = false;
            }
            lastOutput = message.output || "";
            render();
          }

          if (message.type === "resetStarted") {
            running = true;
            lastOutput = "Resetting course files and progress...";
            render();
          }

          if (message.type === "resetFinished") {
            running = false;
            if (message.ok) {
              Object.keys(state.passed).forEach((key) => delete state.passed[key]);
              currentIndex = 0;
            }
            lastOutput = message.output || "";
            render();
          }
        });

        function escapeHtmlClient(value) {
          return String(value)
            .replaceAll("&", "&amp;")
            .replaceAll("<", "&lt;")
            .replaceAll(">", "&gt;")
            .replaceAll('"', "&quot;")
            .replaceAll("'", "&#039;");
        }

        document.addEventListener("click", (event) => {
          const target = event.target;
          if (!target || typeof target.closest !== "function") {
            return;
          }

          const link = target.closest("[data-external-link]");
          if (!link) {
            return;
          }

          event.preventDefault();
          vscode.postMessage({
            type: "openExternal",
            url: link.getAttribute("href")
          });
        });

        render();
      </script>
    `;

    return this.page(webview, body, nonce);
  }

  page(webview, body, nonce = getNonce()) {
    const csp = [
      "default-src 'none'",
      `img-src ${webview.cspSource}`,
      `style-src ${webview.cspSource} 'unsafe-inline'`,
      `script-src 'nonce-${nonce}'`
    ].join("; ");

    return `<!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="UTF-8">
          <meta http-equiv="Content-Security-Policy" content="${csp}">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <style>
            :root {
              --course-ink: #080b0f;
              --course-accent: #18d9ff;
              --course-accent-2: #c8ff3d;
              --course-success: #42f5a7;
              --course-accent-3: #8b5cff;
              --course-danger: #ff5f8f;
              --course-warning: #ffc857;
              --course-border: color-mix(in srgb, var(--vscode-panel-border) 70%, var(--course-accent));
              --course-soft: color-mix(in srgb, var(--vscode-editor-background) 88%, var(--course-accent));
              --course-raised: color-mix(in srgb, var(--vscode-sideBar-background) 86%, var(--course-accent-3));
            }

            body {
              color: var(--vscode-foreground);
              font-family: var(--vscode-font-family);
              margin: 0;
              background:
                linear-gradient(135deg, color-mix(in srgb, var(--course-accent) 8%, transparent), transparent 40%),
                var(--vscode-editor-background);
            }

            main {
              box-sizing: border-box;
              padding: 16px;
            }

            h1 {
              font-size: 15px;
              line-height: 1.3;
              margin: 0 0 6px;
            }

            h2 {
              font-size: 18px;
              line-height: 1.3;
              margin: 0 0 10px;
            }

            h3 {
              font-size: 14px;
              line-height: 1.3;
              margin: 16px 0 8px;
            }

            p {
              line-height: 1.45;
              margin: 0 0 10px;
            }

            ul {
              margin: 0 0 14px;
              padding-left: 20px;
            }

            li {
              line-height: 1.45;
              margin: 0 0 6px;
            }

            code {
              background: var(--vscode-textCodeBlock-background);
              border-radius: 3px;
              font-family: var(--vscode-editor-font-family);
              font-size: 0.95em;
              padding: 1px 4px;
            }

            a {
              color: inherit;
            }

            .lessonPane a {
              color: var(--course-accent);
              text-decoration: underline;
              text-underline-offset: 2px;
            }

            .lessonPane a:hover {
              color: var(--course-accent-2);
            }

            label {
              display: block;
              font-size: 12px;
              margin-bottom: 4px;
            }

            select,
            button,
            summary {
              font: inherit;
            }

            select {
              background: var(--vscode-dropdown-background);
              border: 1px solid var(--course-border);
              border-radius: 10px;
              color: var(--vscode-dropdown-foreground);
              min-height: 34px;
              padding: 6px 10px;
              width: 100%;
            }

            button,
            summary {
              border: 1px solid transparent;
              border-radius: 999px;
              box-sizing: border-box;
              cursor: pointer;
              display: inline-block;
              font-weight: 600;
              letter-spacing: 0;
              min-height: 34px;
              padding: 8px 14px;
              transition: border-color 120ms ease, filter 120ms ease, transform 120ms ease;
            }

            button:hover:not(:disabled),
            summary:hover {
              filter: brightness(1.08);
              transform: translateY(-1px);
            }

            button:focus-visible,
            summary:focus-visible,
            select:focus-visible {
              outline: 2px solid var(--course-accent);
              outline-offset: 2px;
            }

            .primaryButton {
              background: linear-gradient(135deg, var(--course-accent-2), var(--course-accent) 58%, var(--course-accent-3));
              box-shadow: 0 8px 18px color-mix(in srgb, var(--course-accent) 20%, transparent);
              color: var(--course-ink);
            }

            .secondaryButton,
            .hintToggle,
            summary {
              background: color-mix(in srgb, var(--course-accent) 12%, var(--vscode-button-secondaryBackground));
              border-color: color-mix(in srgb, var(--course-accent) 38%, var(--vscode-panel-border));
              color: var(--vscode-button-secondaryForeground);
            }

            .ghostButton {
              background: transparent;
              border-color: color-mix(in srgb, var(--course-accent-3) 45%, var(--vscode-panel-border));
              color: var(--vscode-foreground);
            }

            .dangerButton {
              background: color-mix(in srgb, var(--course-danger) 18%, var(--vscode-editor-background));
              border-color: color-mix(in srgb, var(--course-danger) 65%, var(--vscode-panel-border));
              color: color-mix(in srgb, var(--course-danger) 85%, var(--vscode-foreground));
            }

            .courseHeader {
              border-bottom: 1px solid var(--course-border);
              margin-bottom: 14px;
              padding-bottom: 10px;
            }

            .brandRow {
              align-items: flex-start;
              display: flex;
              flex-wrap: wrap;
              gap: 8px;
              justify-content: space-between;
            }

            .brandTitle {
              align-items: center;
              display: flex;
              gap: 8px;
              min-width: 0;
            }

            .brandMark {
              display: block;
              flex: 0 0 auto;
              height: 24px;
              width: 24px;
            }

            .courseHeader h1 {
              background: linear-gradient(90deg, var(--course-accent-2), var(--course-accent), var(--course-accent-3));
              color: var(--vscode-foreground);
              font-size: 18px;
              font-weight: 800;
              min-width: 0;
            }

            @supports ((-webkit-background-clip: text) or (background-clip: text)) {
              .courseHeader h1 {
                -webkit-background-clip: text;
                background-clip: text;
                color: transparent;
              }
            }

            .courseHeader p {
              color: var(--vscode-descriptionForeground);
              margin-bottom: 0;
            }

            .courseLayout {
              display: grid;
              gap: 14px;
            }

            .lessonPane,
            .controlPane {
              border: 1px solid var(--course-border);
              border-radius: 8px;
              box-sizing: border-box;
              min-width: 0;
              padding: 14px;
            }

            .lessonPane {
              background: color-mix(in srgb, var(--vscode-editor-background) 94%, var(--course-accent));
            }

            .lessonPane h1 {
              font-size: 22px;
              margin-bottom: 12px;
            }

            .lessonPane pre {
              background: var(--vscode-textCodeBlock-background);
              overflow: auto;
              padding: 10px;
              white-space: pre;
            }

            .controlPane {
              align-self: start;
              background:
                linear-gradient(180deg, color-mix(in srgb, var(--course-accent-3) 12%, transparent), transparent 45%),
                var(--vscode-sideBar-background);
              order: -1;
            }

            button:disabled {
              cursor: default;
              opacity: 0.45;
            }

            .actions,
            .links {
              display: flex;
              flex-wrap: wrap;
              gap: 6px;
              margin: 12px 0;
            }

            .hintList {
              display: grid;
              gap: 8px;
              margin: 12px 0;
            }

            .status {
              border: 1px solid color-mix(in srgb, var(--course-warning) 60%, var(--vscode-panel-border));
              border-radius: 999px;
              display: inline-flex;
              margin-top: 8px;
              padding: 4px 9px;
            }

            .status.passed {
              border-color: color-mix(in srgb, var(--course-success) 70%, var(--vscode-panel-border));
              color: color-mix(in srgb, var(--course-success) 80%, var(--vscode-foreground));
            }

            .hintDisclosure,
            .solution {
              width: 100%;
            }

            .hintDisclosure.open .hintToggle,
            .solution[open] summary {
              border-color: color-mix(in srgb, var(--course-accent-2) 54%, var(--vscode-panel-border));
              color: color-mix(in srgb, var(--course-accent-2) 84%, var(--vscode-foreground));
            }

            .hintDisclosure.disabled {
              opacity: 0.48;
            }

            .hintDisclosure p {
              background: color-mix(in srgb, var(--course-accent) 9%, var(--vscode-editor-background));
              border: 1px solid color-mix(in srgb, var(--course-accent) 35%, var(--vscode-panel-border));
              border-left: 4px solid var(--course-accent);
              border-radius: 8px;
              margin: 8px 0 0;
              padding: 9px 10px;
            }

            .hintToggle,
            .solution summary {
              box-sizing: border-box;
              width: fit-content;
            }

            .solution pre,
            #output {
              background: color-mix(in srgb, var(--vscode-textCodeBlock-background) 88%, var(--course-accent-3));
              border: 1px solid color-mix(in srgb, var(--course-accent-3) 35%, transparent);
              border-radius: 8px;
              overflow: auto;
              padding: 10px;
              white-space: pre-wrap;
            }

            .progressCard {
              background: linear-gradient(
                135deg,
                color-mix(in srgb, var(--vscode-sideBar-background) 82%, var(--course-accent)),
                color-mix(in srgb, var(--vscode-sideBar-background) 88%, var(--course-accent-3))
              );
              border: 1px solid color-mix(in srgb, var(--course-accent) 45%, var(--vscode-panel-border));
              border-radius: 8px;
              margin-bottom: 14px;
              padding: 12px;
            }

            .progressTop {
              align-items: center;
              display: flex;
              justify-content: space-between;
              margin-bottom: 8px;
            }

            .progressTop span,
            .progressMeta {
              color: var(--vscode-descriptionForeground);
              font-size: 12px;
            }

            .progressTop strong {
              color: var(--course-accent-2);
              font-size: 20px;
            }

            .progressTrack {
              background: color-mix(in srgb, var(--vscode-editor-background) 72%, var(--course-accent-3));
              border-radius: 999px;
              height: 9px;
              overflow: hidden;
            }

            .progressFill {
              background: linear-gradient(90deg, var(--course-accent-2), var(--course-accent));
              border-radius: inherit;
              height: 100%;
            }

            .progressMeta {
              margin-top: 8px;
            }

            .exerciseDots {
              display: flex;
              flex-wrap: wrap;
              gap: 7px;
              margin-top: 12px;
            }

            .dot {
              align-items: center;
              background: transparent;
              border: 1px solid color-mix(in srgb, var(--vscode-panel-border) 70%, var(--course-accent-3));
              color: var(--vscode-foreground);
              display: inline-flex;
              height: 30px;
              justify-content: center;
              min-height: 30px;
              padding: 0;
              width: 30px;
            }

            .dot.done {
              background: color-mix(in srgb, var(--course-accent-2) 20%, var(--vscode-editor-background));
              border-color: var(--course-accent-2);
              color: var(--vscode-foreground);
            }

            .dot.active {
              box-shadow: 0 0 0 2px color-mix(in srgb, var(--course-accent) 35%, transparent);
              border-color: var(--course-accent);
            }

            @media (min-width: 760px) {
              .courseLayout {
                grid-template-columns: minmax(0, 1fr) minmax(280px, 360px);
              }

              .controlPane {
                order: 0;
                position: sticky;
                top: 12px;
              }
            }
          </style>
        </head>
        <body>${body}</body>
      </html>`;
  }

  webviewOptions() {
    return {
      enableScripts: true,
      localResourceRoots: [vscode.Uri.joinPath(this.context.extensionUri, "media")]
    };
  }
}

function loadLessons(root, course) {
  const lessons = {};

  for (const exercise of course.exercise || []) {
    if (!exercise.lesson) {
      continue;
    }

    const lessonPath = path.join(root, exercise.lesson);
    try {
      lessons[exercise.id] = renderMarkdown(fs.readFileSync(lessonPath, "utf8"));
    } catch (error) {
      lessons[exercise.id] = `<p>${escapeHtml(error.message)}</p>`;
    }
  }

  return lessons;
}

function renderMarkdown(markdown) {
  const lines = String(markdown).replace(/\r\n/g, "\n").split("\n");
  const html = [];
  let paragraph = [];
  let list = [];
  let codeBlock = [];
  let inCodeBlock = false;

  const flushParagraph = () => {
    if (paragraph.length === 0) {
      return;
    }

    html.push(`<p>${renderInlineMarkdown(paragraph.join(" "))}</p>`);
    paragraph = [];
  };

  const flushList = () => {
    if (list.length === 0) {
      return;
    }

    html.push(`<ul>${list.map((item) => `<li>${renderInlineMarkdown(item)}</li>`).join("")}</ul>`);
    list = [];
  };

  const flushCodeBlock = () => {
    html.push(`<pre><code>${escapeHtml(codeBlock.join("\n"))}</code></pre>`);
    codeBlock = [];
  };

  for (const line of lines) {
    const trimmed = line.trim();

    if (trimmed.startsWith("```")) {
      if (inCodeBlock) {
        flushCodeBlock();
        inCodeBlock = false;
      } else {
        flushParagraph();
        flushList();
        inCodeBlock = true;
      }
      continue;
    }

    if (inCodeBlock) {
      codeBlock.push(line);
      continue;
    }

    if (!trimmed) {
      flushParagraph();
      flushList();
      continue;
    }

    const heading = trimmed.match(/^(#{1,3})\s+(.+)$/);
    if (heading) {
      flushParagraph();
      flushList();
      const level = heading[1].length;
      html.push(`<h${level}>${renderInlineMarkdown(heading[2])}</h${level}>`);
      continue;
    }

    const bullet = trimmed.match(/^-\s+(.+)$/);
    if (bullet) {
      flushParagraph();
      list.push(bullet[1]);
      continue;
    }

    flushList();
    paragraph.push(trimmed);
  }

  if (inCodeBlock) {
    flushCodeBlock();
  }
  flushParagraph();
  flushList();

  return html.join("\n");
}

function renderInlineMarkdown(value) {
  const input = String(value);
  const linkPattern = /\[([^\]]+)\]\((https?:\/\/[^)\s]+)\)/g;
  let output = "";
  let cursor = 0;

  for (const match of input.matchAll(linkPattern)) {
    output += renderInlineMarkdownWithoutLinks(input.slice(cursor, match.index));
    const label = match[1];
    const url = match[2];

    if (isAllowedExternalUrl(url)) {
      output += `<a href="${escapeHtml(url)}" data-external-link="true">${renderInlineMarkdownWithoutLinks(label)}</a>`;
    } else {
      output += renderInlineMarkdownWithoutLinks(label);
    }

    cursor = match.index + match[0].length;
  }

  output += renderInlineMarkdownWithoutLinks(input.slice(cursor));
  return output;
}

function renderInlineMarkdownWithoutLinks(value) {
  return escapeHtml(value)
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
}

function isAllowedExternalUrl(url) {
  const allowedHosts = new Set([
    "anza.xyz",
    "docs.surfpool.run",
    "docs.rs",
    "github.com",
    "solana.com",
    "www.anza.xyz",
    "www.solana.com"
  ]);

  try {
    const parsed = new URL(String(url));
    return parsed.protocol === "https:" && allowedHosts.has(parsed.hostname);
  } catch (_error) {
    return false;
  }
}

function findCourseRoot(context) {
  const workspaceFolders = vscode.workspace.workspaceFolders || [];

  for (const folder of workspaceFolders) {
    const candidate = folder.uri.fsPath;
    if (fs.existsSync(path.join(candidate, "course.toml"))) {
      return candidate;
    }
  }

  const parent = path.resolve(context.extensionPath, "..");
  if (fs.existsSync(path.join(parent, "course.toml"))) {
    return parent;
  }

  return undefined;
}

function runCourseRunner(root, args) {
  return new Promise((resolve, reject) => {
    execFile(
      "cargo",
      ["run", "-p", "course-runner", "--", ...args],
      {
        cwd: root,
        maxBuffer: 1024 * 1024 * 8
      },
      (error, stdout, stderr) => {
        if (error) {
          error.output = `${stdout}${stderr}`;
          reject(error);
          return;
        }

        resolve({ stdout, stderr });
      }
    );
  });
}

function isInsidePath(candidate, parent) {
  const relative = path.relative(parent, candidate);
  return relative && !relative.startsWith("..") && !path.isAbsolute(relative);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function escapeScriptJson(value) {
  return JSON.stringify(value).replaceAll("<", "\\u003c");
}

function getNonce() {
  let text = "";
  const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

  for (let i = 0; i < 32; i += 1) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }

  return text;
}

module.exports = {
  activate,
  deactivate
};
