# Project Creation Wizard — Design Document

**Date**: 2026-03-08
**Status**: Approved

## Overview

Replace the single-form project creation with three entry points: a guided wizard (tutorial-style for beginners), an import wizard (for existing Eleventy sites), and an advanced quick-create form.

## Entry Points

From the dashboard, three options:

| Button | Flow | Audience |
|--------|------|----------|
| "Create Your Site" | Guided wizard (7 steps) | Beginners |
| "Open Existing Site" | Import wizard (detect + enhance) | Existing Eleventy users |
| "Advanced Create" | Quick form with selects | Experienced users |

## Guided Wizard

Style: **Conversational cards** — big emoji, full explanation, card-style options with recommended badges. Each step teaches a concept before asking for a choice.

### Step 1: Welcome

- Intro: "Let's build your corner of the web"
- Three cards: Create new / Open existing / Advanced
- Branches to import or advanced flows if selected

### Step 2: Your Identity

- Teaches h-card: "On the indieweb, your identity lives on YOUR domain. Your h-card is like a digital business card that other sites can read."
- Fields: Name, URL, Email (optional), Photo (optional)
- Live preview of rendered h-card

### Step 3: Site Basics

- Site name, directory picker (native)
- Teaches: "This creates a folder on your computer with everything your site needs. You own these files — no cloud lock-in."

### Step 4: Template Language

- Teaches: mail merge analogy for templates
- Cards: Nunjucks (recommended), Liquid, WebC
- Each shows a tiny syntax preview

### Step 5: Styling

- Teaches: what CSS is and the different tools
- Cards: Vanilla CSS (recommended for beginners), Tailwind, Sass

### Step 6: The IndieWeb

Full teaching section:

**Intro**: "The IndieWeb is a movement to take back your content from big platforms. Instead of posting on Twitter and hoping it stays there, you publish on your own site first, then optionally share to social media."

**Core principles**: Own your data, POSSE, connected but independent

**Protocol toggles** (each with explanation, all default ON):

- **Webmention**: "Like @mentions but across websites. When someone replies to your post from their blog, you'll know."
- **Micropub**: "Post to your site from any app — not just Twelvety. It's like having an API for your blog."
- **IndieAuth**: "Sign in to other sites using your own domain. Your URL is your username."

Note: "These are placeholders for now. We'll set up the real endpoints later."

### Step 7: Review & Create

- Summary card with all choices
- Shows what will be created (file count, path)
- "Create Site" button

## Import Wizard (Open Existing)

1. **Pick directory** — Native file picker
2. **Detection** — Scans for eleventy.config.js / .eleventy.js, detects template language, counts content files, checks for indieweb markup
3. **Report** — Shows what was found (e.g., "Nunjucks templates, 12 posts, Sass. No indieweb markup.")
4. **Enhance** — Offers optional additions:
   - twelvety.config.js
   - h-card to layout
   - h-entry to post template
   - Feed templates (Atom, JSON)
   - Webmention/Micropub/IndieAuth link tags
   - Each with explanation and checkbox
5. **Register** — Adds to dashboard

## Advanced Create

The current quick form (select fields for all options), moved to its own route. Minimal explanations, fast path.

## Component Architecture

```
src/routes/new/
├── +page.svelte          # Router: decides which flow
├── wizard/
│   ├── +page.svelte      # Wizard shell: progress dots, navigation
│   ├── Welcome.svelte
│   ├── Identity.svelte
│   ├── SiteBasics.svelte
│   ├── TemplateLang.svelte
│   ├── Styling.svelte
│   ├── IndieWeb.svelte
│   └── Review.svelte
├── import/
│   ├── +page.svelte      # Import flow
│   └── DetectionReport.svelte
└── advanced/
    └── +page.svelte      # Current quick form
```

## State Management

Single Svelte store accumulates wizard choices across steps:

```ts
interface WizardState {
  step: number;
  identity: { name: string; url: string; email: string; photo: string };
  site: { name: string; directory: string };
  templateLang: string;
  css: string;
  indieweb: { webmention: boolean; micropub: boolean; indieauth: boolean };
}
```

Navigation: Back/Next buttons. Back preserves state. Progress dots show completed steps.

## Visual Style

- Dark theme (matches app)
- One concept per screen
- Large emoji illustrations
- Explanation text in muted color, key terms in white/bold
- Card-style options with hover states and "Recommended" badges
- Progress dots at top
