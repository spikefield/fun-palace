# Twelvety Design Document

**Date**: 2026-03-08
**Status**: Approved

## Overview

Twelvety is a native cross-platform desktop application (Windows, macOS, Linux) that generates and manages static sites for the Eleventy static site generator. It is an indieweb-first application — all generated sites ship with proper indieweb markup and the app progressively adds indieweb protocol support.

## Technology Stack

- **App framework**: Tauri v2 (Rust backend + OS webview)
- **Frontend**: SvelteKit (static/SSG mode via adapter-static)
- **Site generator**: Eleventy, bundled as a Node sidecar (advanced users can point to their own Node)
- **CI/CD**: GitHub Actions
- **Development workflow**: Git worktrees for parallel feature development

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Twelvety App                          │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │              SvelteKit Frontend (SSG)              │  │
│  │  Project Manager │ Content Editor │ Deploy/Settings │  │
│  │                     │ Tauri IPC                     │  │
│  └─────────────────────┼──────────────────────────────┘  │
│                        │                                  │
│  ┌─────────────────────┼──────────────────────────────┐  │
│  │              Rust Backend (Tauri)                   │  │
│  │  Process Manager │ IndieWeb Engine │ Filesystem     │  │
│  └───────┬────────────────────────────────────────────┘  │
│          │                                                │
│  ┌───────┴───────┐                                       │
│  │ Node Sidecar  │  (bundled minimal Node runtime)       │
│  │  └─ Eleventy  │                                       │
│  └───────────────┘                                       │
└──────────────────────────────────────────────────────────┘
```

### Responsibility Split

**Rust backend owns:**
- Filesystem operations
- Eleventy process lifecycle (start, stop, restart, kill)
- IndieWeb protocols: Webmention send/receive, Micropub server, IndieAuth, microformats parsing
- Performance-sensitive work: feed parsing, Webmention crawling
- Deploy adapter execution (SFTP via ssh2 crate)

**SvelteKit frontend owns:**
- All UI state, routing, and rendering
- Project configuration UI
- Content editing interface
- Deployment configuration and orchestration
- User-facing logic

**Tauri IPC** is the only bridge between frontend and backend.

## Monorepo Structure

```
twelvety/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC command handlers
│   │   ├── indieweb/       # Webmention, Micropub, microformats
│   │   ├── process/        # Eleventy process management
│   │   └── deploy/         # Deployment adapters
│   └── Cargo.toml
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Svelte stores for app state
│   │   └── utils/          # Frontend utilities
│   └── routes/             # SvelteKit pages
├── templates/              # Eleventy starter templates
├── package.json
├── svelte.config.js
├── tauri.conf.json
└── CLAUDE.md
```

## Project Management

### Project Creation Flow

1. User picks name and directory
2. Chooses from sensible defaults (can customize each):
   - **Template language**: Nunjucks (default), Liquid, Handlebars, JavaScript, WebC
   - **CSS approach**: Vanilla CSS (default), Tailwind, Sass, PostCSS
   - **Content format**: Markdown (default) with frontmatter schema
   - **Starter kit**: Blank, Blog, Portfolio, Personal site
3. App scaffolds project with indieweb markup baked into all templates
4. Project config stored in `twelvety.config.js` (also reads `.twelvety.js`)

### Config File Resolution

1. `twelvety.config.js` (default, preferred)
2. `.twelvety.js` (supported alternative)

First found wins.

### Example `twelvety.config.js`

```js
export default {
  name: "My Site",
  eleventyVersion: "3.0",
  templateLang: "nunjucks",
  css: "vanilla",
  deploy: {
    target: "netlify",
    siteId: "abc123",
  },
  indieweb: {
    domain: "example.com",
    author: { name: "...", url: "...", photo: "..." },
    micropub: true,
    webmention: true,
  },
};
```

Rust reads this by spawning a quick Node one-liner to evaluate and serialize to JSON.

### Eleventy Process Management (Rust)

- `eleventy_serve` — spawns `eleventy --serve`, pipes stdout/stderr to UI
- `eleventy_build` — spawns `eleventy --build`, captures output, returns success/failure + stats
- Process lifecycle fully managed — start, stop, restart, kill on app close

### Multi-Project Support

- App maintains a registry of known projects (stored in app data dir)
- Dashboard shows all projects with status (last built, deploy status)
- One project active at a time for preview; can trigger builds/deploys on any

## Deployment

### Deploy Adapter Trait

```rust
trait DeployAdapter {
    fn validate_config(&self, config: &DeployConfig) -> Result<()>;
    fn deploy(&self, build_dir: &Path, config: &DeployConfig) -> Result<DeployResult>;
    fn get_status(&self, config: &DeployConfig) -> Result<DeployStatus>;
}
```

### Targets

| Target | Auth | Mechanism |
|--------|------|-----------|
| Netlify | OAuth / PAT | Netlify CLI / API |
| Cloudflare Pages | API token | Wrangler CLI / Direct Upload API |
| Vercel | OAuth | Vercel CLI / API |
| GitHub Pages | Git auth | Push to configured branch |
| SFTP | Host/user/pass or SSH key | Rust ssh2 crate |

### Deploy Flow

1. User configures target in project settings (stored in `twelvety.config.js`)
2. Credentials stored in OS keychain via Tauri secure storage
3. "Deploy" triggers: build → validate → deploy via adapter → show status
4. Deploy history kept locally

### GitHub Actions Integration

- App can generate `.github/workflows/deploy.yml`
- "Set up CI" wizard writes workflow file and guides setting secrets

## IndieWeb Engine

### Phase 1 — Baked Into Templates

All starter templates ship with:
- `h-card` on homepage/about (author identity)
- `h-entry` on all post templates
- `h-feed` on listing pages
- `rel-me` links in head/footer
- RSS, Atom, and JSON Feed via Eleventy templates
- `<link>` tags for Webmention, Micropub, IndieAuth endpoints

### Phase 2 — Content Types

| Post Type | h-entry Properties | UI |
|-----------|-------------------|-----|
| Article | `p-name`, `e-content`, `dt-published` | Full editor with title |
| Note | `e-content`, `dt-published` | Quick compose, no title |
| Reply | `e-content`, `u-in-reply-to` | Compose + reply URL |
| Like | `u-like-of` | Just a URL |
| Bookmark | `u-bookmark-of`, `p-name`, `e-content` | URL + optional comment |
| Repost | `u-repost-of` | Just a URL |
| RSVP | `p-rsvp`, `u-in-reply-to` | Event URL + yes/no/maybe/interested |
| Check-in | `p-location`, `e-content` | Location + optional note |

Each type maps to a frontmatter schema and template. Content saved as Markdown files.

### Phase 3 — Protocols (Rust)

| Protocol | Responsibility | How |
|----------|---------------|-----|
| Webmention | Send & receive | Scan new content for outbound links → send. Receive endpoint stores in SQLite, exposed via Eleventy data files |
| Micropub | Server | Local HTTP server accepting Micropub → creates Markdown → triggers rebuild |
| IndieAuth | Client & server | Sign in with domain. Client for third-party services, optional provider |
| Microsub | Client | Feed reader in-app — subscribe to h-feeds and RSS via Rust feed parsing |
| POSSE | Syndication | After publish, syndicate to configured targets (initially via Bridgy or manual) |

### Rust Crates

- `microformats` — parse mf2 from HTML
- `webmention` — discover and send Webmentions
- `indieweb` — higher-level protocol support
- `feed-rs` — parse RSS/Atom/JSON feeds

## Data Storage

### App-Level (OS app data directory)

| Data | Storage | Reason |
|------|---------|--------|
| Project registry | `projects.json` | List of known projects |
| Credentials | OS Keychain (Tauri plugin) | Never on disk |
| App preferences | `settings.json` | Theme, defaults, Node path override |
| Webmention inbox | SQLite (per project) | Structured queries, can grow large |
| Deploy history | SQLite (per project) | Queryable deploy log |
| Microsub feeds | SQLite (shared) | Feed subscriptions and cache |

### Project-Level (in Eleventy project directory)

| Data | Storage | Reason |
|------|---------|--------|
| Project config | `twelvety.config.js` or `.twelvety.js` | Checked into git, portable |
| Content | Markdown files | Standard Eleventy, no lock-in |
| Eleventy config | `eleventy.config.js` | Standard Eleventy |
| Webmention data | `_data/webmentions.json` | Eleventy data cascade |
| Syndication links | Frontmatter per post | Stays with content |

### Zero Lock-In Principle

If a user stops using Twelvety, they have a standard Eleventy project with Markdown files. `twelvety.config.js` is the only app-specific file, and it's optional.

### SvelteKit State

- Svelte stores for reactive UI state
- Tauri event system for push updates from Rust (build progress, Webmention received)

## Error Handling & Offline Behavior

### Error Strategy

| Layer | Approach |
|-------|----------|
| Rust commands | `Result<T, TwelvetyError>` unified error enum → user-friendly messages |
| Tauri IPC | Errors serialize as `{ code, message, context }` |
| SvelteKit | Toasts for transient errors, inline for form errors, dedicated views for fatal |
| Eleventy process | Stderr parsed → structured log panel |

### Offline Behavior

Local-first by design. Everything works offline except:
- Deploying
- Sending/receiving Webmentions
- Microsub feed fetching
- IndieAuth

Offline actions queue and execute when connectivity returns.

## Testing & CI

### Testing Strategy

| Layer | Framework | Scope |
|-------|-----------|-------|
| Rust unit | `cargo test` | Adapters, protocols, process mgmt, config parsing |
| Rust integration | `cargo test` | Tauri commands with mock filesystem |
| SvelteKit unit | Vitest | Stores, utilities, component logic |
| SvelteKit components | Vitest + Testing Library | Render and interaction |
| E2E | Playwright (Tauri WebDriver) | Full app flows |

### GitHub Actions Pipeline

```yaml
on: [push, pull_request]

jobs:
  test-rust:     # cargo fmt, clippy, test
  test-frontend: # vitest
  build:         # tauri build (matrix: ubuntu, macos, windows)
  e2e:           # Playwright against built app
  release:       # On tag → build installers → GitHub Release
```

### Release Artifacts

- Windows: `.msi` / `.exe`
- macOS: `.dmg` / `.app`
- Linux: `.deb` / `.AppImage`

## Implementation Phases

1. **Phase 1 — Project management & build pipeline**: Scaffold Eleventy projects from templates (indieweb markup baked in), preview, build, deploy
2. **Phase 2 — Content authoring**: Markdown/frontmatter editor for all indieweb post types, media management
3. **Phase 3 — IndieWeb protocols**: Micropub, Webmention, IndieAuth, Microsub, POSSE automation

## Development Workflow

- Git worktrees for all feature work — parallel development without conflicts
- GitHub Actions CI on every push/PR
- Cross-platform build matrix
