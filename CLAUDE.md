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
- Worktree directory: `.worktrees/` (gitignored)
- TDD: write failing test → implement → verify → commit
- Keep commits small and focused
