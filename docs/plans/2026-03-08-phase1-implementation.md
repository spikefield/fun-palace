# Phase 1: Project Management & Build Pipeline — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a working Tauri v2 desktop app that can create Eleventy projects from templates (with indieweb markup), preview/build them, and deploy to free-tier hosts.

**Architecture:** Tauri v2 (Rust) + SvelteKit (adapter-static, SSG mode) monorepo. Eleventy runs as a Node sidecar process managed by the Rust backend. Frontend communicates with backend exclusively via Tauri IPC commands and events.

**Tech Stack:** Rust, Tauri v2, SvelteKit 2, Svelte 5, TypeScript, Vitest, Eleventy 3.x, Nunjucks

---

## Task 1: Scaffold Tauri v2 + SvelteKit Project

**Files:**
- Create: `package.json`, `svelte.config.js`, `vite.config.ts`, `tsconfig.json`
- Create: `src/routes/+layout.ts`, `src/routes/+page.svelte`, `src/app.html`
- Create: `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/src/lib.rs`, `src-tauri/src/main.rs`
- Create: `src-tauri/capabilities/default.json`
- Create: `.gitignore`

**Step 1: Create the project using Tauri scaffolding**

```bash
npm create tauri-app@latest twelvety-init -- --template sveltekit-ts
```

Copy the generated files into our repo root (since we already have the git repo with the design doc). Alternatively, scaffold in a temp dir and move files.

**Step 2: Install adapter-static and configure SvelteKit for SPA mode**

```bash
npm install --save-dev @sveltejs/adapter-static
```

`svelte.config.js`:
```js
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: 'index.html',
    }),
  },
};

export default config;
```

`src/routes/+layout.ts`:
```ts
export const ssr = false;
```

**Step 3: Configure tauri.conf.json build paths**

Ensure `frontendDist` points to `../build` (SvelteKit adapter-static output):
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}
```

**Step 4: Install Tauri plugins we'll need**

```bash
cd src-tauri
cargo add tauri-plugin-shell tauri-plugin-dialog tauri-plugin-fs
cd ..
npm add @tauri-apps/plugin-shell @tauri-apps/plugin-dialog @tauri-apps/plugin-fs
```

Register plugins in `src-tauri/src/lib.rs`:
```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 5: Add capabilities/permissions**

`src-tauri/capabilities/default.json`:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for Twelvety",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default",
    "fs:default",
    "shell:allow-execute",
    "shell:allow-spawn"
  ]
}
```

**Step 6: Verify it builds and runs**

```bash
npm run tauri dev
```

Expected: A Tauri window opens showing the default SvelteKit page.

**Step 7: Create .gitignore**

```
node_modules/
build/
.svelte-kit/
src-tauri/target/
_site/
.DS_Store
*.env
*.env.*
```

**Step 8: Commit**

```bash
git add -A
git commit -m "feat: scaffold Tauri v2 + SvelteKit project"
```

---

## Task 2: Set Up CLAUDE.md

**Files:**
- Create: `CLAUDE.md`

**Step 1: Write CLAUDE.md with project conventions**

```markdown
# Twelvety

Native cross-platform desktop app for generating and managing Eleventy static sites with indieweb support.

## Tech Stack
- **App framework**: Tauri v2 (Rust backend)
- **Frontend**: SvelteKit 2 + Svelte 5 (adapter-static, SSG/SPA mode)
- **Site generator**: Eleventy 3.x (Node sidecar)
- **Testing**: cargo test (Rust), Vitest (frontend)

## Project Structure
- `src/` — SvelteKit frontend
- `src-tauri/` — Rust backend (Tauri)
- `src-tauri/src/commands/` — IPC command handlers
- `templates/` — Eleventy starter templates
- `docs/plans/` — Design and implementation plans

## Commands
- `npm run dev` — Start SvelteKit dev server
- `npm run tauri dev` — Start full Tauri app in dev mode
- `npm run tauri build` — Build release binaries
- `npm test` — Run frontend tests (Vitest)
- `cd src-tauri && cargo test` — Run Rust tests
- `cd src-tauri && cargo clippy` — Lint Rust code
- `cd src-tauri && cargo fmt --check` — Check Rust formatting

## Conventions
- Rust: snake_case, `Result<T, TwelvetyError>` for all commands
- TypeScript: camelCase, strict mode
- Svelte: Svelte 5 runes syntax ($state, $derived, $effect)
- All Tauri IPC commands defined in `src-tauri/src/commands/` modules
- Frontend calls backend via `invoke()` from `@tauri-apps/api/core`
- Config resolution order: `twelvety.config.js` → `.twelvety.js`
- Eleventy config: `eleventy.config.js` (ESM)
- All starter templates include indieweb microformats markup

## Development Workflow
- Use git worktrees for all feature branches
- TDD: write failing test → implement → verify → commit
- Keep commits small and focused
```

**Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add CLAUDE.md with project conventions"
```

---

## Task 3: Set Up Testing Infrastructure

**Files:**
- Create: `vitest.config.ts`
- Create: `src/lib/__tests__/smoke.test.ts`
- Create: `src-tauri/src/error.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `package.json` (add test script)

**Step 1: Install Vitest and testing dependencies**

```bash
npm install --save-dev vitest @testing-library/svelte @testing-library/jest-dom jsdom
```

**Step 2: Create vitest.config.ts**

```ts
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['src/**/*.test.ts'],
    environment: 'jsdom',
    globals: true,
  },
});
```

**Step 3: Add test script to package.json**

Add to `"scripts"`:
```json
"test": "vitest run",
"test:watch": "vitest"
```

**Step 4: Write a smoke test**

`src/lib/__tests__/smoke.test.ts`:
```ts
import { describe, it, expect } from 'vitest';

describe('smoke test', () => {
  it('should pass', () => {
    expect(1 + 1).toBe(2);
  });
});
```

**Step 5: Run frontend test**

```bash
npm test
```

Expected: 1 test passes.

**Step 6: Create Rust error type**

`src-tauri/src/error.rs`:
```rust
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct TwelvetyError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for TwelvetyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for TwelvetyError {}

impl From<std::io::Error> for TwelvetyError {
    fn from(err: std::io::Error) -> Self {
        TwelvetyError {
            code: "IO_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for TwelvetyError {
    fn from(err: serde_json::Error) -> Self {
        TwelvetyError {
            code: "JSON_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}
```

Add `serde_json` dependency:
```bash
cd src-tauri && cargo add serde_json
```

**Step 7: Wire error module into lib.rs**

Add to `src-tauri/src/lib.rs`:
```rust
mod error;
pub use error::TwelvetyError;
```

**Step 8: Run Rust tests**

```bash
cd src-tauri && cargo test
```

Expected: Compiles and passes (no tests yet, but no errors).

**Step 9: Commit**

```bash
git add -A
git commit -m "feat: set up testing infrastructure (Vitest + cargo test)"
```

---

## Task 4: Project Registry — Rust Backend

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/projects.rs`
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/project.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`

The project registry tracks all known Eleventy projects. Stored as `projects.json` in the Tauri app data directory.

**Step 1: Write the failing test for ProjectRegistry**

`src-tauri/src/models/project.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: String,
    pub template_lang: String,
    pub css_approach: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectRegistry {
    pub projects: Vec<ProjectEntry>,
}

impl ProjectRegistry {
    pub fn load(path: &std::path::Path) -> Result<Self, crate::TwelvetyError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        let registry: ProjectRegistry = serde_json::from_str(&data)?;
        Ok(registry)
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), crate::TwelvetyError> {
        let data = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn add(&mut self, entry: ProjectEntry) {
        self.projects.push(entry);
    }

    pub fn remove(&mut self, id: &str) {
        self.projects.retain(|p| p.id != id);
    }

    pub fn find(&self, id: &str) -> Option<&ProjectEntry> {
        self.projects.iter().find(|p| p.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_registry_load_empty() {
        let path = Path::new("/tmp/twelvety_test_nonexistent.json");
        let registry = ProjectRegistry::load(path).unwrap();
        assert!(registry.projects.is_empty());
    }

    #[test]
    fn test_registry_add_and_find() {
        let mut registry = ProjectRegistry::default();
        let entry = ProjectEntry {
            id: "test-1".to_string(),
            name: "Test Project".to_string(),
            path: PathBuf::from("/tmp/test"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        };
        registry.add(entry);
        assert_eq!(registry.projects.len(), 1);
        assert!(registry.find("test-1").is_some());
        assert!(registry.find("nonexistent").is_none());
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = ProjectRegistry::default();
        registry.add(ProjectEntry {
            id: "rm-1".to_string(),
            name: "Remove Me".to_string(),
            path: PathBuf::from("/tmp/rm"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        });
        registry.remove("rm-1");
        assert!(registry.projects.is_empty());
    }

    #[test]
    fn test_registry_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("projects.json");

        let mut registry = ProjectRegistry::default();
        registry.add(ProjectEntry {
            id: "save-1".to_string(),
            name: "Saved".to_string(),
            path: PathBuf::from("/tmp/saved"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        });
        registry.save(&path).unwrap();

        let loaded = ProjectRegistry::load(&path).unwrap();
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].name, "Saved");
    }
}
```

Add tempfile dev dependency:
```bash
cd src-tauri && cargo add --dev tempfile
```

**Step 2: Run tests to verify they pass**

```bash
cd src-tauri && cargo test
```

Expected: All 4 tests pass.

**Step 3: Create IPC commands for project management**

`src-tauri/src/commands/mod.rs`:
```rust
pub mod projects;
```

`src-tauri/src/commands/projects.rs`:
```rust
use crate::models::project::{ProjectEntry, ProjectRegistry};
use crate::TwelvetyError;
use tauri::AppHandle;
use tauri::Manager;

fn registry_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("projects.json")
}

#[tauri::command]
pub async fn list_projects(app: AppHandle) -> Result<Vec<ProjectEntry>, TwelvetyError> {
    let path = registry_path(&app);
    let registry = ProjectRegistry::load(&path)?;
    Ok(registry.projects)
}

#[tauri::command]
pub async fn add_project(app: AppHandle, entry: ProjectEntry) -> Result<(), TwelvetyError> {
    let path = registry_path(&app);
    let mut registry = ProjectRegistry::load(&path)?;
    registry.add(entry);
    registry.save(&path)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_project(app: AppHandle, id: String) -> Result<(), TwelvetyError> {
    let path = registry_path(&app);
    let mut registry = ProjectRegistry::load(&path)?;
    registry.remove(&id);
    registry.save(&path)?;
    Ok(())
}
```

`src-tauri/src/models/mod.rs`:
```rust
pub mod project;
```

**Step 4: Register commands in lib.rs**

```rust
mod commands;
mod error;
mod models;

pub use error::TwelvetyError;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::projects::list_projects,
            commands::projects::add_project,
            commands::projects::remove_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 5: Run tests and cargo clippy**

```bash
cd src-tauri && cargo test && cargo clippy
```

Expected: All tests pass, no clippy warnings.

**Step 6: Commit**

```bash
git add -A
git commit -m "feat: add project registry with IPC commands"
```

---

## Task 5: Config File Resolution — Rust

**Files:**
- Create: `src-tauri/src/config.rs`
- Modify: `src-tauri/src/lib.rs`

Reads `twelvety.config.js` or `.twelvety.js` from a project directory by spawning a quick Node eval.

**Step 1: Write the config resolver with tests**

`src-tauri/src/config.rs`:
```rust
use crate::TwelvetyError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The resolved Twelvety project config
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TwelvetyConfig {
    pub name: Option<String>,
    #[serde(rename = "eleventyVersion")]
    pub eleventy_version: Option<String>,
    #[serde(rename = "templateLang")]
    pub template_lang: Option<String>,
    pub css: Option<String>,
    pub deploy: Option<DeployConfig>,
    pub indieweb: Option<IndiewebConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeployConfig {
    pub target: Option<String>,
    #[serde(rename = "siteId")]
    pub site_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndiewebConfig {
    pub domain: Option<String>,
    pub author: Option<AuthorConfig>,
    pub micropub: Option<bool>,
    pub webmention: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
}

/// Find the config file path in a project directory.
/// Resolution order: twelvety.config.js → .twelvety.js
pub fn find_config_file(project_dir: &Path) -> Option<PathBuf> {
    let candidates = ["twelvety.config.js", ".twelvety.js"];
    for name in &candidates {
        let path = project_dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Read the config by evaluating the JS file with Node.
/// Returns TwelvetyConfig parsed from the JSON output.
pub fn read_config(project_dir: &Path, node_path: &str) -> Result<TwelvetyConfig, TwelvetyError> {
    let config_path = find_config_file(project_dir).ok_or_else(|| TwelvetyError {
        code: "CONFIG_NOT_FOUND".to_string(),
        message: format!(
            "No twelvety.config.js or .twelvety.js found in {}",
            project_dir.display()
        ),
    })?;

    let script = format!(
        r#"import('{url}').then(m => console.log(JSON.stringify(m.default || m)))"#,
        url = config_path.display().to_string().replace('\\', "/")
    );

    let output = std::process::Command::new(node_path)
        .arg("--input-type=module")
        .arg("-e")
        .arg(&script)
        .current_dir(project_dir)
        .output()
        .map_err(|e| TwelvetyError {
            code: "NODE_EXEC_ERROR".to_string(),
            message: format!("Failed to run Node: {}", e),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TwelvetyError {
            code: "CONFIG_EVAL_ERROR".to_string(),
            message: format!("Failed to evaluate config: {}", stderr),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let config: TwelvetyConfig = serde_json::from_str(stdout.trim())?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_config_prefers_twelvety_config_js() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("twelvety.config.js"), "").unwrap();
        fs::write(dir.path().join(".twelvety.js"), "").unwrap();

        let found = find_config_file(dir.path()).unwrap();
        assert!(found.ends_with("twelvety.config.js"));
    }

    #[test]
    fn test_find_config_falls_back_to_dotfile() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".twelvety.js"), "").unwrap();

        let found = find_config_file(dir.path()).unwrap();
        assert!(found.ends_with(".twelvety.js"));
    }

    #[test]
    fn test_find_config_returns_none_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_config_file(dir.path()).is_none());
    }

    #[test]
    fn test_read_config_parses_js_export() {
        // This test requires Node to be installed
        if std::process::Command::new("node")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("Skipping: Node not available");
            return;
        }

        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("twelvety.config.js"),
            r#"export default { name: "Test Site", templateLang: "nunjucks" };"#,
        )
        .unwrap();

        let config = read_config(dir.path(), "node").unwrap();
        assert_eq!(config.name.unwrap(), "Test Site");
        assert_eq!(config.template_lang.unwrap(), "nunjucks");
    }
}
```

**Step 2: Wire into lib.rs**

Add `mod config;` to `src-tauri/src/lib.rs`.

**Step 3: Run tests**

```bash
cd src-tauri && cargo test
```

Expected: All tests pass (config Node test may skip if Node not available on CI — that's fine).

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: add config file resolution (twelvety.config.js / .twelvety.js)"
```

---

## Task 6: Eleventy Process Manager — Rust

**Files:**
- Create: `src-tauri/src/process.rs`
- Create: `src-tauri/src/commands/eleventy.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

Manages Eleventy child processes (serve, build, stop). Uses Tauri events to stream output to the frontend.

**Step 1: Write the process manager**

`src-tauri/src/process.rs`:
```rust
use crate::TwelvetyError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

pub struct ProcessManager {
    processes: Mutex<HashMap<String, Child>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Mutex::new(HashMap::new()),
        }
    }

    /// Start `eleventy --serve` for a project. Returns the PID.
    pub fn serve(
        &self,
        project_id: &str,
        project_dir: &PathBuf,
        node_path: &str,
        eleventy_path: &str,
    ) -> Result<u32, TwelvetyError> {
        let mut procs = self.processes.lock().unwrap();

        // Kill existing process for this project if running
        if let Some(mut child) = procs.remove(project_id) {
            let _ = child.kill();
        }

        let child = Command::new(node_path)
            .arg(eleventy_path)
            .arg("--serve")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| TwelvetyError {
                code: "PROCESS_SPAWN_ERROR".to_string(),
                message: format!("Failed to start Eleventy: {}", e),
            })?;

        let pid = child.id();
        procs.insert(project_id.to_string(), child);
        Ok(pid)
    }

    /// Run `eleventy --build` and wait for completion.
    pub fn build(
        &self,
        project_dir: &PathBuf,
        node_path: &str,
        eleventy_path: &str,
    ) -> Result<String, TwelvetyError> {
        let output = Command::new(node_path)
            .arg(eleventy_path)
            .current_dir(project_dir)
            .output()
            .map_err(|e| TwelvetyError {
                code: "BUILD_ERROR".to_string(),
                message: format!("Failed to run Eleventy build: {}", e),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TwelvetyError {
                code: "BUILD_FAILED".to_string(),
                message: format!("Eleventy build failed:\n{}", stderr),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    /// Stop the dev server for a project.
    pub fn stop(&self, project_id: &str) -> Result<(), TwelvetyError> {
        let mut procs = self.processes.lock().unwrap();
        if let Some(mut child) = procs.remove(project_id) {
            child.kill().map_err(|e| TwelvetyError {
                code: "PROCESS_KILL_ERROR".to_string(),
                message: format!("Failed to stop process: {}", e),
            })?;
        }
        Ok(())
    }

    /// Stop all running processes (called on app shutdown).
    pub fn stop_all(&self) {
        let mut procs = self.processes.lock().unwrap();
        for (_, mut child) in procs.drain() {
            let _ = child.kill();
        }
    }

    /// Check if a project has a running dev server.
    pub fn is_running(&self, project_id: &str) -> bool {
        let procs = self.processes.lock().unwrap();
        procs.contains_key(project_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_is_running_false_initially() {
        let pm = ProcessManager::new();
        assert!(!pm.is_running("test-project"));
    }

    #[test]
    fn test_process_manager_stop_nonexistent_is_ok() {
        let pm = ProcessManager::new();
        assert!(pm.stop("nonexistent").is_ok());
    }

    #[test]
    fn test_process_manager_stop_all_empty() {
        let pm = ProcessManager::new();
        pm.stop_all(); // should not panic
    }
}
```

**Step 2: Create IPC commands for Eleventy**

`src-tauri/src/commands/eleventy.rs`:
```rust
use crate::models::project::ProjectRegistry;
use crate::process::ProcessManager;
use crate::TwelvetyError;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn eleventy_serve(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<u32, TwelvetyError> {
    let registry_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| TwelvetyError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();
    // TODO: resolve node and eleventy paths from settings/sidecar
    let node_path = "node";
    let eleventy_path = "npx";

    process_manager.serve(&project_id, &project_dir, node_path, eleventy_path)
}

#[tauri::command]
pub async fn eleventy_build(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<String, TwelvetyError> {
    let registry_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| TwelvetyError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();
    let node_path = "node";
    let eleventy_path = "npx";

    process_manager.build(&project_dir, node_path, eleventy_path)
}

#[tauri::command]
pub async fn eleventy_stop(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<(), TwelvetyError> {
    process_manager.stop(&project_id)
}

#[tauri::command]
pub async fn eleventy_status(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<bool, TwelvetyError> {
    Ok(process_manager.is_running(&project_id))
}
```

**Step 3: Update commands/mod.rs and lib.rs**

`src-tauri/src/commands/mod.rs`:
```rust
pub mod eleventy;
pub mod projects;
```

Update `lib.rs` to manage ProcessManager state and register commands:
```rust
mod commands;
mod config;
mod error;
mod models;
mod process;

pub use error::TwelvetyError;

use process::ProcessManager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(ProcessManager::new())
        .invoke_handler(tauri::generate_handler![
            commands::projects::list_projects,
            commands::projects::add_project,
            commands::projects::remove_project,
            commands::eleventy::eleventy_serve,
            commands::eleventy::eleventy_build,
            commands::eleventy::eleventy_stop,
            commands::eleventy::eleventy_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 4: Run tests**

```bash
cd src-tauri && cargo test && cargo clippy
```

Expected: All tests pass, no clippy warnings.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add Eleventy process manager with serve/build/stop commands"
```

---

## Task 7: Eleventy Blog Starter Template with IndieWeb Markup

**Files:**
- Create: `templates/blog/eleventy.config.js`
- Create: `templates/blog/package.json`
- Create: `templates/blog/src/_data/site.json`
- Create: `templates/blog/src/_includes/base.njk`
- Create: `templates/blog/src/_includes/post.njk`
- Create: `templates/blog/src/index.njk`
- Create: `templates/blog/src/about.njk`
- Create: `templates/blog/src/posts/posts.json`
- Create: `templates/blog/src/posts/hello-world.md`
- Create: `templates/blog/twelvety.config.js`

**Step 1: Create the Eleventy config**

`templates/blog/eleventy.config.js`:
```js
import { feedPlugin } from "@11ty/eleventy-plugin-rss";

export default function (eleventyConfig) {
  const siteData = {
    title: "My Site",
    url: "https://example.com",
    author: { name: "Your Name" },
  };

  eleventyConfig.addPlugin(feedPlugin, {
    type: "atom",
    outputPath: "/feed.xml",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: "en",
      title: siteData.title,
      subtitle: "",
      base: siteData.url,
      author: siteData.author,
    },
  });

  eleventyConfig.addPlugin(feedPlugin, {
    type: "json",
    outputPath: "/feed.json",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: "en",
      title: siteData.title,
      subtitle: "",
      base: siteData.url,
      author: siteData.author,
    },
  });

  return {
    dir: {
      input: "src",
      output: "_site",
      includes: "_includes",
      data: "_data",
    },
    templateFormats: ["md", "njk", "html"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
  };
}
```

**Step 2: Create package.json**

`templates/blog/package.json`:
```json
{
  "name": "twelvety-blog",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "eleventy --serve",
    "build": "eleventy"
  },
  "dependencies": {
    "@11ty/eleventy": "^3.0.0",
    "@11ty/eleventy-plugin-rss": "^2.0.0"
  }
}
```

**Step 3: Create site data**

`templates/blog/src/_data/site.json`:
```json
{
  "title": "My Site",
  "url": "https://example.com",
  "description": "A personal site on the indieweb",
  "language": "en",
  "author": {
    "name": "Your Name",
    "url": "https://example.com",
    "email": "you@example.com",
    "photo": "/img/avatar.jpg"
  }
}
```

**Step 4: Create base layout with indieweb markup**

`templates/blog/src/_includes/base.njk`:
```html
<!DOCTYPE html>
<html lang="{{ site.language | default('en') }}">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{% if title %}{{ title }} — {% endif %}{{ site.title }}</title>
  <meta name="description" content="{{ description | default(site.description) }}">

  <!-- IndieWeb endpoints -->
  <link rel="indieauth-metadata" href="{{ site.url }}/.well-known/oauth-authorization-server">
  <link rel="authorization_endpoint" href="https://indieauth.com/auth">
  <link rel="token_endpoint" href="https://tokens.indieauth.com/token">
  <link rel="webmention" href="{{ site.url }}/webmention">
  <link rel="micropub" href="{{ site.url }}/micropub">

  <!-- Feeds -->
  <link rel="alternate" type="application/atom+xml" title="Atom Feed" href="/feed.xml">
  <link rel="alternate" type="application/feed+json" title="JSON Feed" href="/feed.json">

  <!-- rel-me for identity verification -->
  {% if site.author.url %}<link rel="me" href="{{ site.author.url }}">{% endif %}
</head>
<body>
  <header>
    <nav>
      <a href="/" class="site-title">{{ site.title }}</a>
      <a href="/about/">About</a>
    </nav>
  </header>

  <main>
    {{ content | safe }}
  </main>

  <footer>
    <!-- h-card: author identity -->
    <div class="h-card" rel="author">
      {% if site.author.photo %}
      <img class="u-photo" src="{{ site.author.photo }}" alt="{{ site.author.name }}">
      {% endif %}
      <a class="p-name u-url u-uid" rel="me" href="{{ site.author.url }}">{{ site.author.name }}</a>
      {% if site.author.email %}
      <a class="u-email" href="mailto:{{ site.author.email }}">{{ site.author.email }}</a>
      {% endif %}
    </div>
  </footer>
</body>
</html>
```

**Step 5: Create post layout with h-entry**

`templates/blog/src/_includes/post.njk`:
```html
---
layout: base.njk
---
<article class="h-entry">
  <header>
    <h1 class="p-name">{{ title }}</h1>
    <time class="dt-published" datetime="{{ date | dateToRfc3339 }}">
      {{ date | readableDate }}
    </time>
    <a class="p-author h-card" href="{{ site.author.url }}">{{ site.author.name }}</a>
    <a class="u-url" href="{{ page.url }}" hidden></a>
  </header>

  <div class="e-content">
    {{ content | safe }}
  </div>

  {% if syndication %}
  <div class="syndication">
    {% for link in syndication %}
    <a class="u-syndication" rel="syndication" href="{{ link }}">{{ link }}</a>
    {% endfor %}
  </div>
  {% endif %}
</article>
```

**Step 6: Create homepage with h-feed**

`templates/blog/src/index.njk`:
```html
---
layout: base.njk
title: Home
---
<section class="h-feed">
  <h1 class="p-name">{{ site.title }}</h1>

  {%- for post in collections.posts | reverse %}
  <article class="h-entry">
    <h2 class="p-name">
      <a class="u-url" href="{{ post.url }}">{{ post.data.title }}</a>
    </h2>
    <time class="dt-published" datetime="{{ post.date | dateToRfc3339 }}">
      {{ post.date | readableDate }}
    </time>
    {% if post.data.description %}
    <p class="p-summary">{{ post.data.description }}</p>
    {% endif %}
  </article>
  {%- endfor %}
</section>
```

**Step 7: Create about page with full h-card**

`templates/blog/src/about.njk`:
```html
---
layout: base.njk
title: About
---
<div class="h-card">
  {% if site.author.photo %}
  <img class="u-photo" src="{{ site.author.photo }}" alt="{{ site.author.name }}">
  {% endif %}
  <h1 class="p-name">{{ site.author.name }}</h1>
  <p class="p-note">This is your personal bio. Edit it in <code>src/about.njk</code>.</p>
  <a class="u-url u-uid" rel="me" href="{{ site.author.url }}">{{ site.author.url }}</a>
  {% if site.author.email %}
  <a class="u-email" href="mailto:{{ site.author.email }}">{{ site.author.email }}</a>
  {% endif %}
</div>
```

**Step 8: Create directory data and sample post**

`templates/blog/src/posts/posts.json`:
```json
{
  "layout": "post.njk",
  "tags": "posts",
  "permalink": "/posts/{{ page.fileSlug }}/"
}
```

`templates/blog/src/posts/hello-world.md`:
```markdown
---
title: Hello World
description: Welcome to your new indieweb site, powered by Twelvety and Eleventy.
date: 2026-01-01
---

Welcome to your new site! This blog is built with [Eleventy](https://www.11ty.dev/) and managed by [Twelvety](https://github.com/twelvety/twelvety).

Your site already supports the [IndieWeb](https://indieweb.org/) — it includes microformats2 markup (h-card, h-entry, h-feed), feeds (Atom and JSON Feed), and placeholders for Webmention and Micropub endpoints.
```

**Step 9: Create default twelvety.config.js**

`templates/blog/twelvety.config.js`:
```js
export default {
  name: "My Site",
  eleventyVersion: "3.0",
  templateLang: "nunjucks",
  css: "vanilla",
  deploy: {
    target: "github-pages",
  },
  indieweb: {
    domain: "example.com",
    author: {
      name: "Your Name",
      url: "https://example.com",
      photo: "/img/avatar.jpg",
    },
    micropub: false,
    webmention: false,
  },
};
```

**Step 10: Commit**

```bash
git add templates/
git commit -m "feat: add blog starter template with indieweb microformats markup"
```

---

## Task 8: Project Scaffolding Command — Rust

**Files:**
- Create: `src-tauri/src/commands/scaffold.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

Copies a template to a new directory, customizes site data, and registers the project.

**Step 1: Write the scaffold module with tests**

`src-tauri/src/commands/scaffold.rs`:
```rust
use crate::models::project::{ProjectEntry, ProjectRegistry};
use crate::TwelvetyError;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Deserialize)]
pub struct ScaffoldOptions {
    pub name: String,
    pub directory: PathBuf,
    pub starter: String,       // "blog", "blank", "portfolio", "personal"
    pub template_lang: String, // "nunjucks", "liquid", "webc"
    pub css: String,           // "vanilla", "tailwind", "sass"
    pub author_name: String,
    pub author_url: String,
    pub site_url: String,
}

/// Copy directory recursively
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), TwelvetyError> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Replace placeholder values in site.json
fn customize_site_data(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), TwelvetyError> {
    let site_json_path = project_dir.join("src/_data/site.json");
    if site_json_path.exists() {
        let mut data: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&site_json_path)?)?;

        if let Some(obj) = data.as_object_mut() {
            obj.insert("title".into(), serde_json::json!(opts.name));
            obj.insert("url".into(), serde_json::json!(opts.site_url));
            if let Some(author) = obj.get_mut("author").and_then(|a| a.as_object_mut()) {
                author.insert("name".into(), serde_json::json!(opts.author_name));
                author.insert("url".into(), serde_json::json!(opts.author_url));
            }
        }

        std::fs::write(&site_json_path, serde_json::to_string_pretty(&data)?)?;
    }
    Ok(())
}

/// Write the twelvety.config.js with user choices
fn write_twelvety_config(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), TwelvetyError> {
    let config = format!(
        r#"export default {{
  name: "{}",
  eleventyVersion: "3.0",
  templateLang: "{}",
  css: "{}",
  deploy: {{
    target: "github-pages",
  }},
  indieweb: {{
    domain: "{}",
    author: {{
      name: "{}",
      url: "{}",
    }},
    micropub: false,
    webmention: false,
  }},
}};
"#,
        opts.name,
        opts.template_lang,
        opts.css,
        opts.site_url.replace("https://", "").replace("http://", ""),
        opts.author_name,
        opts.author_url,
    );
    std::fs::write(project_dir.join("twelvety.config.js"), config)?;
    Ok(())
}

#[tauri::command]
pub async fn scaffold_project(
    app: AppHandle,
    options: ScaffoldOptions,
) -> Result<ProjectEntry, TwelvetyError> {
    let template_dir = app
        .path()
        .resource_dir()
        .unwrap()
        .join("templates")
        .join(&options.starter);

    if !template_dir.exists() {
        return Err(TwelvetyError {
            code: "TEMPLATE_NOT_FOUND".to_string(),
            message: format!("Starter template '{}' not found", options.starter),
        });
    }

    // Copy template to project directory
    copy_dir_recursive(&template_dir, &options.directory)?;

    // Customize
    customize_site_data(&options.directory, &options)?;
    write_twelvety_config(&options.directory, &options)?;

    // Register project
    let entry = ProjectEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name: options.name.clone(),
        path: options.directory.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        template_lang: options.template_lang.clone(),
        css_approach: options.css.clone(),
    };

    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let mut registry = ProjectRegistry::load(&registry_path)?;
    registry.add(entry.clone());
    registry.save(&registry_path)?;

    Ok(entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dir_recursive() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        let dst_path = dst.path().join("output");

        // Create source structure
        std::fs::write(src.path().join("file.txt"), "hello").unwrap();
        std::fs::create_dir(src.path().join("sub")).unwrap();
        std::fs::write(src.path().join("sub/nested.txt"), "world").unwrap();

        copy_dir_recursive(src.path(), &dst_path).unwrap();

        assert!(dst_path.join("file.txt").exists());
        assert!(dst_path.join("sub/nested.txt").exists());
        assert_eq!(std::fs::read_to_string(dst_path.join("file.txt")).unwrap(), "hello");
    }

    #[test]
    fn test_customize_site_data() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src/_data")).unwrap();
        std::fs::write(
            dir.path().join("src/_data/site.json"),
            r#"{"title":"placeholder","url":"https://example.com","author":{"name":"placeholder","url":"https://example.com"}}"#,
        ).unwrap();

        let opts = ScaffoldOptions {
            name: "Cool Blog".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "nunjucks".to_string(),
            css: "vanilla".to_string(),
            author_name: "Alice".to_string(),
            author_url: "https://alice.example".to_string(),
            site_url: "https://alice.example".to_string(),
        };

        customize_site_data(dir.path(), &opts).unwrap();

        let data: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(dir.path().join("src/_data/site.json")).unwrap(),
        ).unwrap();

        assert_eq!(data["title"], "Cool Blog");
        assert_eq!(data["author"]["name"], "Alice");
    }
}
```

Add `uuid` and `chrono` dependencies:
```bash
cd src-tauri && cargo add uuid --features v4 && cargo add chrono
```

**Step 2: Register scaffold command**

`src-tauri/src/commands/mod.rs`:
```rust
pub mod eleventy;
pub mod projects;
pub mod scaffold;
```

Add to `lib.rs` invoke_handler:
```rust
commands::scaffold::scaffold_project,
```

**Step 3: Configure Tauri to bundle the templates directory**

In `src-tauri/tauri.conf.json`, add to the `bundle` section:
```json
{
  "bundle": {
    "resources": ["../templates/**/*"]
  }
}
```

**Step 4: Run tests**

```bash
cd src-tauri && cargo test && cargo clippy
```

Expected: All tests pass.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add project scaffolding with template copying and customization"
```

---

## Task 9: SvelteKit — App Shell & Routing

**Files:**
- Create: `src/routes/+layout.svelte`
- Modify: `src/routes/+page.svelte`
- Create: `src/routes/project/[id]/+page.svelte`
- Create: `src/routes/project/[id]/+page.ts`
- Create: `src/routes/new/+page.svelte`
- Create: `src/lib/stores/projects.ts`
- Create: `src/lib/types.ts`
- Create: `src/app.css`

**Step 1: Define TypeScript types**

`src/lib/types.ts`:
```ts
export interface ProjectEntry {
  id: string;
  name: string;
  path: string;
  created_at: string;
  template_lang: string;
  css_approach: string;
}

export interface ScaffoldOptions {
  name: string;
  directory: string;
  starter: string;
  template_lang: string;
  css: string;
  author_name: string;
  author_url: string;
  site_url: string;
}

export interface TwelvetyError {
  code: string;
  message: string;
}
```

**Step 2: Create the projects store**

`src/lib/stores/projects.ts`:
```ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ProjectEntry } from '$lib/types';

export const projects = writable<ProjectEntry[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export async function loadProjects() {
  loading.set(true);
  error.set(null);
  try {
    const result = await invoke<ProjectEntry[]>('list_projects');
    projects.set(result);
  } catch (e: unknown) {
    const err = e as { message?: string };
    error.set(err.message ?? 'Failed to load projects');
  } finally {
    loading.set(false);
  }
}

export async function removeProject(id: string) {
  await invoke('remove_project', { id });
  await loadProjects();
}
```

**Step 3: Write the test for the types module**

`src/lib/__tests__/types.test.ts`:
```ts
import { describe, it, expect } from 'vitest';
import type { ProjectEntry, ScaffoldOptions } from '$lib/types';

describe('types', () => {
  it('ProjectEntry shape is correct', () => {
    const project: ProjectEntry = {
      id: '123',
      name: 'Test',
      path: '/tmp/test',
      created_at: '2026-03-08',
      template_lang: 'nunjucks',
      css_approach: 'vanilla',
    };
    expect(project.id).toBe('123');
    expect(project.name).toBe('Test');
  });

  it('ScaffoldOptions shape is correct', () => {
    const opts: ScaffoldOptions = {
      name: 'Blog',
      directory: '/tmp/blog',
      starter: 'blog',
      template_lang: 'nunjucks',
      css: 'vanilla',
      author_name: 'Alice',
      author_url: 'https://alice.example',
      site_url: 'https://alice.example',
    };
    expect(opts.starter).toBe('blog');
  });
});
```

**Step 4: Run frontend tests**

```bash
npm test
```

Expected: Tests pass.

**Step 5: Create global styles**

`src/app.css`:
```css
:root {
  --color-bg: #fafafa;
  --color-surface: #ffffff;
  --color-text: #1a1a1a;
  --color-text-muted: #6b7280;
  --color-primary: #4f46e5;
  --color-primary-hover: #4338ca;
  --color-border: #e5e7eb;
  --color-danger: #dc2626;
  --radius: 8px;
  --shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #111111;
    --color-surface: #1a1a1a;
    --color-text: #f5f5f5;
    --color-text-muted: #9ca3af;
    --color-primary: #818cf8;
    --color-primary-hover: #6366f1;
    --color-border: #2d2d2d;
    --color-danger: #f87171;
  }
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: var(--color-bg);
  color: var(--color-text);
  line-height: 1.5;
}

button {
  cursor: pointer;
  border: none;
  border-radius: var(--radius);
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  transition: background 0.15s;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}
.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-danger {
  background: var(--color-danger);
  color: white;
}
```

**Step 6: Create the root layout**

`src/routes/+layout.svelte`:
```svelte
<script>
  import '../app.css';
  let { children } = $props();
</script>

<div class="app">
  {@render children()}
</div>

<style>
  .app {
    max-width: 960px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
</style>
```

**Step 7: Create the dashboard (home page)**

`src/routes/+page.svelte`:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { projects, loading, error, loadProjects, removeProject } from '$lib/stores/projects';

  onMount(() => {
    loadProjects();
  });
</script>

<header>
  <h1>Twelvety</h1>
  <a href="/new"><button class="btn-primary">New Project</button></a>
</header>

{#if $loading}
  <p>Loading projects...</p>
{:else if $error}
  <p class="error">{$error}</p>
{:else if $projects.length === 0}
  <div class="empty">
    <p>No projects yet.</p>
    <a href="/new"><button class="btn-primary">Create your first site</button></a>
  </div>
{:else}
  <div class="project-list">
    {#each $projects as project}
      <div class="project-card">
        <div>
          <h2><a href="/project/{project.id}">{project.name}</a></h2>
          <p class="meta">{project.template_lang} &middot; {project.css_approach}</p>
          <p class="path">{project.path}</p>
        </div>
        <button class="btn-danger" onclick={() => removeProject(project.id)}>Remove</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  .empty {
    text-align: center;
    padding: 4rem 0;
  }
  .project-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .project-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 1rem 1.5rem;
    box-shadow: var(--shadow);
  }
  .project-card h2 { font-size: 1.125rem; }
  .project-card h2 a { color: var(--color-text); text-decoration: none; }
  .project-card h2 a:hover { color: var(--color-primary); }
  .meta { color: var(--color-text-muted); font-size: 0.875rem; }
  .path { color: var(--color-text-muted); font-size: 0.75rem; font-family: monospace; }
  .error { color: var(--color-danger); }
</style>
```

**Step 8: Create the new project page (placeholder)**

`src/routes/new/+page.svelte`:
```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import type { ProjectEntry, ScaffoldOptions } from '$lib/types';

  let name = $state('');
  let directory = $state('');
  let starter = $state('blog');
  let templateLang = $state('nunjucks');
  let css = $state('vanilla');
  let authorName = $state('');
  let authorUrl = $state('');
  let siteUrl = $state('');
  let creating = $state(false);
  let errorMsg = $state('');

  async function pickDirectory() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      directory = selected as string;
    }
  }

  async function createProject() {
    if (!name || !directory) {
      errorMsg = 'Name and directory are required.';
      return;
    }
    creating = true;
    errorMsg = '';
    try {
      const options: ScaffoldOptions = {
        name,
        directory: `${directory}/${name.toLowerCase().replace(/\s+/g, '-')}`,
        starter,
        template_lang: templateLang,
        css,
        author_name: authorName,
        author_url: authorUrl,
        site_url: siteUrl,
      };
      const project = await invoke<ProjectEntry>('scaffold_project', { options });
      goto(`/project/${project.id}`);
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to create project';
    } finally {
      creating = false;
    }
  }
</script>

<header>
  <a href="/">&larr; Back</a>
  <h1>New Project</h1>
</header>

<form onsubmit={(e) => { e.preventDefault(); createProject(); }}>
  <label>
    Site Name
    <input type="text" bind:value={name} placeholder="My Blog" required>
  </label>

  <label>
    Directory
    <div class="dir-picker">
      <input type="text" bind:value={directory} placeholder="/home/user/sites" readonly>
      <button type="button" class="btn-primary" onclick={pickDirectory}>Browse</button>
    </div>
  </label>

  <label>
    Starter Template
    <select bind:value={starter}>
      <option value="blog">Blog</option>
      <option value="blank">Blank</option>
    </select>
  </label>

  <label>
    Template Language
    <select bind:value={templateLang}>
      <option value="nunjucks">Nunjucks</option>
      <option value="liquid">Liquid</option>
      <option value="webc">WebC</option>
    </select>
  </label>

  <label>
    CSS Approach
    <select bind:value={css}>
      <option value="vanilla">Vanilla CSS</option>
      <option value="tailwind">Tailwind</option>
      <option value="sass">Sass</option>
    </select>
  </label>

  <fieldset>
    <legend>Author Info</legend>
    <label>
      Name
      <input type="text" bind:value={authorName} placeholder="Your Name">
    </label>
    <label>
      URL
      <input type="url" bind:value={authorUrl} placeholder="https://example.com">
    </label>
    <label>
      Site URL
      <input type="url" bind:value={siteUrl} placeholder="https://example.com">
    </label>
  </fieldset>

  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}

  <button type="submit" class="btn-primary" disabled={creating}>
    {creating ? 'Creating...' : 'Create Project'}
  </button>
</form>

<style>
  form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    max-width: 480px;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  input, select {
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    font-size: 0.875rem;
    background: var(--color-surface);
    color: var(--color-text);
  }
  .dir-picker {
    display: flex;
    gap: 0.5rem;
  }
  .dir-picker input { flex: 1; }
  fieldset {
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  legend { font-weight: 600; padding: 0 0.5rem; }
  .error { color: var(--color-danger); font-size: 0.875rem; }
  header { margin-bottom: 2rem; }
  header a { color: var(--color-text-muted); text-decoration: none; }
</style>
```

**Step 9: Create the project detail page**

`src/routes/project/[id]/+page.ts`:
```ts
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
  return { projectId: params.id };
};
```

`src/routes/project/[id]/+page.svelte`:
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { ProjectEntry } from '$lib/types';

  let { data } = $props();

  let project = $state<ProjectEntry | null>(null);
  let building = $state(false);
  let serving = $state(false);
  let buildOutput = $state('');
  let errorMsg = $state('');

  async function loadProject() {
    try {
      const projects = await invoke<ProjectEntry[]>('list_projects');
      project = projects.find((p) => p.id === data.projectId) ?? null;
      if (project) {
        serving = await invoke<boolean>('eleventy_status', { projectId: data.projectId });
      }
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to load project';
    }
  }

  async function startServe() {
    try {
      await invoke('eleventy_serve', { projectId: data.projectId });
      serving = true;
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to start dev server';
    }
  }

  async function stopServe() {
    try {
      await invoke('eleventy_stop', { projectId: data.projectId });
      serving = false;
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to stop dev server';
    }
  }

  async function runBuild() {
    building = true;
    buildOutput = '';
    errorMsg = '';
    try {
      const output = await invoke<string>('eleventy_build', { projectId: data.projectId });
      buildOutput = output;
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Build failed';
    } finally {
      building = false;
    }
  }

  $effect(() => {
    loadProject();
  });
</script>

<header>
  <a href="/">&larr; All Projects</a>
</header>

{#if !project}
  <p>Loading project...</p>
{:else}
  <h1>{project.name}</h1>
  <p class="path">{project.path}</p>
  <p class="meta">{project.template_lang} &middot; {project.css_approach}</p>

  <div class="actions">
    {#if serving}
      <button class="btn-danger" onclick={stopServe}>Stop Server</button>
    {:else}
      <button class="btn-primary" onclick={startServe}>Start Dev Server</button>
    {/if}

    <button class="btn-primary" onclick={runBuild} disabled={building}>
      {building ? 'Building...' : 'Build'}
    </button>
  </div>

  {#if errorMsg}
    <pre class="error">{errorMsg}</pre>
  {/if}

  {#if buildOutput}
    <details open>
      <summary>Build Output</summary>
      <pre class="output">{buildOutput}</pre>
    </details>
  {/if}
{/if}

<style>
  header { margin-bottom: 1rem; }
  header a { color: var(--color-text-muted); text-decoration: none; }
  h1 { margin-bottom: 0.25rem; }
  .path { font-family: monospace; font-size: 0.75rem; color: var(--color-text-muted); }
  .meta { color: var(--color-text-muted); font-size: 0.875rem; margin-bottom: 1.5rem; }
  .actions { display: flex; gap: 0.5rem; margin-bottom: 1.5rem; }
  .output, .error {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 1rem;
    font-size: 0.75rem;
    overflow-x: auto;
    white-space: pre-wrap;
  }
  .error { color: var(--color-danger); }
</style>
```

**Step 10: Commit**

```bash
git add src/
git commit -m "feat: add SvelteKit app shell with dashboard, project creation, and project detail pages"
```

---

## Task 10: GitHub Actions CI Pipeline

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: Write the CI workflow**

`.github/workflows/ci.yml`:
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm
      - run: npm ci
      - run: npm test

  test-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: src-tauri
      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
      - run: cd src-tauri && cargo fmt --check
      - run: cd src-tauri && cargo clippy -- -D warnings
      - run: cd src-tauri && cargo test

  build:
    needs: [test-frontend, test-rust]
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: src-tauri
      - name: Install Linux dependencies
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
      - run: npm ci
      - run: npx tauri build
      - uses: actions/upload-artifact@v4
        with:
          name: twelvety-${{ matrix.target }}
          path: |
            src-tauri/target/release/bundle/**/*.deb
            src-tauri/target/release/bundle/**/*.AppImage
            src-tauri/target/release/bundle/**/*.dmg
            src-tauri/target/release/bundle/**/*.msi
            src-tauri/target/release/bundle/**/*.exe
```

**Step 2: Commit**

```bash
git add .github/
git commit -m "ci: add GitHub Actions pipeline with test, lint, and cross-platform build"
```

---

## Task Summary

| Task | What It Builds | Key Files |
|------|---------------|-----------|
| 1 | Tauri + SvelteKit scaffold | Project root, src-tauri/, src/ |
| 2 | CLAUDE.md conventions | CLAUDE.md |
| 3 | Testing infrastructure | vitest.config.ts, error.rs |
| 4 | Project registry CRUD | models/project.rs, commands/projects.rs |
| 5 | Config file resolution | config.rs |
| 6 | Eleventy process manager | process.rs, commands/eleventy.rs |
| 7 | Blog template with indieweb | templates/blog/** |
| 8 | Project scaffolding command | commands/scaffold.rs |
| 9 | SvelteKit app shell + pages | src/routes/**, src/lib/** |
| 10 | GitHub Actions CI | .github/workflows/ci.yml |

## What's NOT in Phase 1

- Content editor UI (Phase 2)
- Deploy adapter implementations beyond the trait (Phase 1.5)
- IndieWeb protocol engines (Phase 3)
- Microsub feed reader (Phase 3)
- Node sidecar bundling (needs build pipeline work — tracked separately)
- Additional starter templates (portfolio, personal, blank)
