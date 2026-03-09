# Project Creation Wizard — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the single-form project creation with a guided tutorial wizard, import wizard for existing sites, and advanced quick-create form.

**Architecture:** Svelte 5 components with a shared wizard store. Each wizard step is a standalone component rendered by a shell that manages progress/navigation. Import wizard adds a Rust `detect_project` command. The existing `scaffold_project` command is reused.

**Tech Stack:** SvelteKit, Svelte 5 runes, TypeScript, Tauri IPC, Vitest

---

## Task 1: Wizard State Store

**Files:**
- Create: `src/lib/stores/wizard.ts`
- Create: `src/lib/__tests__/wizard.test.ts`

**Step 1: Create the wizard store**

`src/lib/stores/wizard.ts`:
```ts
import { writable, derived } from 'svelte/store';

export interface WizardState {
  step: number;
  identity: { name: string; url: string; email: string; photo: string };
  site: { name: string; directory: string };
  templateLang: string;
  css: string;
  indieweb: { webmention: boolean; micropub: boolean; indieauth: boolean };
}

const defaultState: WizardState = {
  step: 0,
  identity: { name: '', url: '', email: '', photo: '' },
  site: { name: '', directory: '' },
  templateLang: 'nunjucks',
  css: 'vanilla',
  indieweb: { webmention: true, micropub: true, indieauth: true },
};

export const wizard = writable<WizardState>({ ...defaultState });

export const totalSteps = 7;

export const progress = derived(wizard, ($w) => $w.step / (totalSteps - 1));

export function nextStep() {
  wizard.update((s) => ({ ...s, step: Math.min(s.step + 1, totalSteps - 1) }));
}

export function prevStep() {
  wizard.update((s) => ({ ...s, step: Math.max(s.step - 1, 0) }));
}

export function goToStep(step: number) {
  wizard.update((s) => ({ ...s, step }));
}

export function resetWizard() {
  wizard.set({ ...defaultState });
}

export function updateIdentity(identity: Partial<WizardState['identity']>) {
  wizard.update((s) => ({ ...s, identity: { ...s.identity, ...identity } }));
}

export function updateSite(site: Partial<WizardState['site']>) {
  wizard.update((s) => ({ ...s, site: { ...s.site, ...site } }));
}

export function updateIndieweb(indieweb: Partial<WizardState['indieweb']>) {
  wizard.update((s) => ({ ...s, indieweb: { ...s.indieweb, ...indieweb } }));
}
```

**Step 2: Write tests**

`src/lib/__tests__/wizard.test.ts`:
```ts
import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import {
  wizard,
  nextStep,
  prevStep,
  goToStep,
  resetWizard,
  updateIdentity,
  updateSite,
  progress,
  totalSteps,
} from '$lib/stores/wizard';

describe('wizard store', () => {
  beforeEach(() => resetWizard());

  it('starts at step 0', () => {
    expect(get(wizard).step).toBe(0);
  });

  it('nextStep increments', () => {
    nextStep();
    expect(get(wizard).step).toBe(1);
  });

  it('prevStep decrements', () => {
    nextStep();
    nextStep();
    prevStep();
    expect(get(wizard).step).toBe(1);
  });

  it('prevStep does not go below 0', () => {
    prevStep();
    expect(get(wizard).step).toBe(0);
  });

  it('nextStep does not exceed max', () => {
    for (let i = 0; i < totalSteps + 5; i++) nextStep();
    expect(get(wizard).step).toBe(totalSteps - 1);
  });

  it('goToStep sets step directly', () => {
    goToStep(3);
    expect(get(wizard).step).toBe(3);
  });

  it('updateIdentity merges partial updates', () => {
    updateIdentity({ name: 'Alice' });
    expect(get(wizard).identity.name).toBe('Alice');
    expect(get(wizard).identity.url).toBe('');
  });

  it('updateSite merges partial updates', () => {
    updateSite({ name: 'My Blog' });
    expect(get(wizard).site.name).toBe('My Blog');
  });

  it('progress is 0 at start', () => {
    expect(get(progress)).toBe(0);
  });

  it('resetWizard returns to defaults', () => {
    nextStep();
    updateIdentity({ name: 'Alice' });
    resetWizard();
    expect(get(wizard).step).toBe(0);
    expect(get(wizard).identity.name).toBe('');
  });
});
```

**Step 3: Run tests**

```bash
npm test
```

**Step 4: Commit**

```bash
git add src/lib/stores/wizard.ts src/lib/__tests__/wizard.test.ts
git commit -m "feat: add wizard state store with navigation and update helpers"
```

---

## Task 2: Wizard Shell & Welcome Step

**Files:**
- Create: `src/routes/new/wizard/+page.svelte`
- Create: `src/lib/components/wizard/WizardShell.svelte`
- Create: `src/lib/components/wizard/Welcome.svelte`
- Modify: `src/routes/new/+page.svelte`

**Step 1: Create WizardShell**

`src/lib/components/wizard/WizardShell.svelte`:
```svelte
<script lang="ts">
  import { wizard, totalSteps, nextStep, prevStep } from '$lib/stores/wizard';

  let { children } = $props();
</script>

<div class="wizard">
  <div class="progress">
    {#each Array(totalSteps) as _, i}
      <div class="dot" class:active={$wizard.step === i} class:done={i < $wizard.step}></div>
    {/each}
  </div>

  <div class="step-label">Step {$wizard.step + 1} of {totalSteps}</div>

  <div class="content">
    {@render children()}
  </div>

  <div class="nav">
    {#if $wizard.step > 0}
      <button class="btn btn-back" onclick={prevStep}>&larr; Back</button>
    {:else}
      <div></div>
    {/if}
    {#if $wizard.step < totalSteps - 1}
      <button class="btn btn-next" onclick={nextStep}>Next &rarr;</button>
    {/if}
  </div>
</div>

<style>
  .wizard { max-width: 600px; margin: 0 auto; }
  .progress { display: flex; gap: 0.5rem; margin-bottom: 1.5rem; }
  .dot { width: 10px; height: 10px; border-radius: 50%; background: var(--color-border); }
  .dot.active { background: var(--color-primary); }
  .dot.done { background: #4ade80; }
  .step-label { font-size: 0.75rem; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 1rem; }
  .content { margin-bottom: 2rem; }
  .nav { display: flex; justify-content: space-between; }
  .btn { padding: 0.75rem 1.5rem; border-radius: var(--radius); font-weight: 600; }
  .btn-back { background: transparent; color: var(--color-text-muted); }
  .btn-next { background: var(--color-primary); color: white; }
  .btn-next:hover { background: var(--color-primary-hover); }
</style>
```

**Step 2: Create Welcome step**

`src/lib/components/wizard/Welcome.svelte`:
```svelte
<script lang="ts">
  import { goToStep } from '$lib/stores/wizard';
  import { goto } from '$app/navigation';

  function startWizard() {
    goToStep(1);
  }
</script>

<div class="illustration">&#127758;</div>
<h1>Let's build your corner of the web</h1>
<p class="explanation">
  Twelvety helps you create a personal website that <strong>you own</strong>.
  No algorithms, no ads, no platform lock-in — just your content, your way.
</p>

<div class="options">
  <button class="option" onclick={startWizard}>
    <div class="icon">&#10024;</div>
    <div>
      <h3>Create a new site</h3>
      <p>I'll guide you through every step and explain what everything means.</p>
    </div>
  </button>

  <button class="option" onclick={() => goto('/new/import')}>
    <div class="icon">&#128194;</div>
    <div>
      <h3>Open an existing Eleventy site</h3>
      <p>I'll detect your setup and offer to add indieweb features.</p>
    </div>
  </button>

  <button class="option" onclick={() => goto('/new/advanced')}>
    <div class="icon">&#9881;&#65039;</div>
    <div>
      <h3>Advanced create</h3>
      <p>I know what I'm doing — just give me the options.</p>
    </div>
  </button>
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; }
  .explanation strong { color: var(--color-text); }
  .options { display: flex; flex-direction: column; gap: 0.75rem; }
  .option {
    display: flex; align-items: center; gap: 1rem; text-align: left;
    background: var(--color-surface); border: 2px solid var(--color-border);
    border-radius: 12px; padding: 1.25rem; cursor: pointer;
    transition: border-color 0.15s; color: var(--color-text); width: 100%;
  }
  .option:hover { border-color: var(--color-primary); }
  .option .icon { font-size: 1.5rem; flex-shrink: 0; }
  .option h3 { font-size: 1rem; margin-bottom: 0.15rem; }
  .option p { color: var(--color-text-muted); font-size: 0.85rem; }
</style>
```

**Step 3: Create wizard page and update router**

`src/routes/new/wizard/+page.svelte`:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { wizard, resetWizard } from '$lib/stores/wizard';
  import WizardShell from '$lib/components/wizard/WizardShell.svelte';
  import Welcome from '$lib/components/wizard/Welcome.svelte';

  onMount(() => resetWizard());
</script>

<WizardShell>
  {#if $wizard.step === 0}
    <Welcome />
  {:else}
    <p>Step {$wizard.step + 1} coming soon...</p>
  {/if}
</WizardShell>
```

Update `src/routes/new/+page.svelte` to redirect to wizard:
```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => goto('/new/wizard'));
</script>

<p>Redirecting...</p>
```

**Step 4: Verify build**

```bash
npm run build
```

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add wizard shell, welcome step, and route restructuring"
```

---

## Task 3: Identity Step

**Files:**
- Create: `src/lib/components/wizard/Identity.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

**Step 1: Create Identity component**

`src/lib/components/wizard/Identity.svelte`:
```svelte
<script lang="ts">
  import { wizard, updateIdentity } from '$lib/stores/wizard';
</script>

<div class="illustration">&#128100;</div>
<h1>Who are you on the web?</h1>
<p class="explanation">
  On the indieweb, <strong>your identity lives on your own domain</strong> — not
  on Twitter, Facebook, or any other platform. Your site will include an
  <strong>h-card</strong>: a machine-readable digital business card that tells
  other websites who you are.
</p>
<p class="explanation">
  Think of it like this: when you comment on someone's blog or reply from your
  site, they'll see your name, photo, and a link back to you — all pulled from
  your h-card automatically.
</p>

<div class="fields">
  <label>
    Your Name <span class="required">*</span>
    <input type="text" value={$wizard.identity.name}
      oninput={(e) => updateIdentity({ name: e.currentTarget.value })}
      placeholder="Alice Example">
  </label>

  <label>
    Your Website URL <span class="required">*</span>
    <input type="url" value={$wizard.identity.url}
      oninput={(e) => updateIdentity({ url: e.currentTarget.value })}
      placeholder="https://alice.example.com">
    <span class="hint">This becomes your identity on the indieweb. Don't have one yet? Pick what you want it to be.</span>
  </label>

  <label>
    Email <span class="optional">(optional)</span>
    <input type="email" value={$wizard.identity.email}
      oninput={(e) => updateIdentity({ email: e.currentTarget.value })}
      placeholder="alice@example.com">
  </label>

  <label>
    Photo URL <span class="optional">(optional)</span>
    <input type="text" value={$wizard.identity.photo}
      oninput={(e) => updateIdentity({ photo: e.currentTarget.value })}
      placeholder="/img/avatar.jpg">
    <span class="hint">A URL to your profile photo. You can change this later.</span>
  </label>
</div>

{#if $wizard.identity.name}
  <div class="preview">
    <div class="preview-label">Preview: Your h-card</div>
    <div class="h-card-preview">
      <strong>{$wizard.identity.name}</strong>
      {#if $wizard.identity.url}
        <span class="url">{$wizard.identity.url}</span>
      {/if}
    </div>
  </div>
{/if}

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1rem; }
  .explanation strong { color: var(--color-text); }
  .fields { display: flex; flex-direction: column; gap: 1rem; margin-bottom: 1.5rem; }
  label { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.875rem; font-weight: 500; }
  input {
    padding: 0.5rem; border: 1px solid var(--color-border); border-radius: var(--radius);
    font-size: 0.875rem; background: var(--color-surface); color: var(--color-text);
  }
  .required { color: var(--color-danger); }
  .optional { color: var(--color-text-muted); font-weight: 400; }
  .hint { font-size: 0.75rem; color: var(--color-text-muted); font-weight: 400; }
  .preview { background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 1rem; }
  .preview-label { font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem; text-transform: uppercase; letter-spacing: 0.05em; }
  .h-card-preview strong { display: block; }
  .url { font-size: 0.85rem; color: var(--color-primary); }
</style>
```

**Step 2: Wire into wizard page**

Add import and step 1 case to `src/routes/new/wizard/+page.svelte`:
```svelte
  import Identity from '$lib/components/wizard/Identity.svelte';

  <!-- in the if/else chain -->
  {:else if $wizard.step === 1}
    <Identity />
```

**Step 3: Verify build, commit**

```bash
npm run build
git add -A
git commit -m "feat: add identity step with h-card explanation and live preview"
```

---

## Task 4: Site Basics Step

**Files:**
- Create: `src/lib/components/wizard/SiteBasics.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

**Step 1: Create SiteBasics component**

`src/lib/components/wizard/SiteBasics.svelte`:
```svelte
<script lang="ts">
  import { wizard, updateSite } from '$lib/stores/wizard';

  async function pickDirectory() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      updateSite({ directory: selected as string });
    }
  }
</script>

<div class="illustration">&#128193;</div>
<h1>Name your site</h1>
<p class="explanation">
  Every site needs a name and a home on your computer. This creates a folder
  with all your site's files — <strong>you own them completely</strong>. No cloud
  service, no subscription. If you ever stop using Twelvety, your files are still
  there as a standard Eleventy project.
</p>

<div class="fields">
  <label>
    Site Name <span class="required">*</span>
    <input type="text" value={$wizard.site.name}
      oninput={(e) => updateSite({ name: e.currentTarget.value })}
      placeholder="Alice's Garden">
  </label>

  <label>
    Save Location <span class="required">*</span>
    <div class="dir-picker">
      <input type="text" value={$wizard.site.directory} placeholder="Choose a folder..." readonly>
      <button class="btn-primary" onclick={pickDirectory}>Browse</button>
    </div>
    <span class="hint">Your site will be created in a subfolder here.</span>
  </label>
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1.5rem; }
  .explanation strong { color: var(--color-text); }
  .fields { display: flex; flex-direction: column; gap: 1rem; }
  label { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.875rem; font-weight: 500; }
  input {
    padding: 0.5rem; border: 1px solid var(--color-border); border-radius: var(--radius);
    font-size: 0.875rem; background: var(--color-surface); color: var(--color-text);
  }
  .dir-picker { display: flex; gap: 0.5rem; }
  .dir-picker input { flex: 1; }
  .required { color: var(--color-danger); }
  .hint { font-size: 0.75rem; color: var(--color-text-muted); font-weight: 400; }
</style>
```

**Step 2: Wire in, build, commit**

```bash
npm run build
git add -A
git commit -m "feat: add site basics step with directory picker"
```

---

## Task 5: Template Language Step

**Files:**
- Create: `src/lib/components/wizard/TemplateLang.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

**Step 1: Create TemplateLang component**

`src/lib/components/wizard/TemplateLang.svelte`:
```svelte
<script lang="ts">
  import { wizard } from '$lib/stores/wizard';

  const options = [
    {
      value: 'nunjucks',
      name: 'Nunjucks',
      recommended: true,
      desc: 'The most popular choice for Eleventy. Powerful features like template inheritance and macros. If you\'re not sure, pick this one.',
      preview: '{{ title }}',
      tag: 'Used by: Google, Mozilla, NASA',
    },
    {
      value: 'liquid',
      name: 'Liquid',
      recommended: false,
      desc: 'Simpler and more beginner-friendly. Originally from Shopify. Great if you\'ve used Jekyll or Shopify themes before.',
      preview: '{{ page.title }}',
      tag: 'Used by: Shopify, Jekyll sites',
    },
    {
      value: 'webc',
      name: 'WebC',
      recommended: false,
      desc: 'Web-native components. Write templates as custom HTML elements. Newer and more experimental, but very clean.',
      preview: '<my-header></my-header>',
      tag: 'Eleventy-native, cutting edge',
    },
  ];

  function select(value: string) {
    wizard.update((s) => ({ ...s, templateLang: value }));
  }
</script>

<div class="illustration">&#127912;</div>
<h1>How should your templates work?</h1>
<p class="explanation">
  Templates control how your content turns into web pages. Think of them like
  <strong>mail merge</strong> — you write your blog post in plain text, and the
  template wraps it in your site's design with headers, navigation, and footer.
  <br><br>
  Different template languages have different syntax for this. Don't worry —
  you can always change later.
</p>

<div class="options">
  {#each options as opt}
    <button
      class="option"
      class:selected={$wizard.templateLang === opt.value}
      onclick={() => select(opt.value)}
    >
      <h3>{opt.name} {#if opt.recommended}<span class="badge">Recommended</span>{/if}</h3>
      <p>{opt.desc}</p>
      <code class="preview">{opt.preview}</code>
      <span class="tag">{opt.tag}</span>
    </button>
  {/each}
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; }
  .explanation strong { color: var(--color-text); }
  .options { display: flex; flex-direction: column; gap: 0.75rem; }
  .option {
    text-align: left; width: 100%; background: var(--color-surface);
    border: 2px solid var(--color-border); border-radius: 12px;
    padding: 1.25rem; cursor: pointer; transition: all 0.15s; color: var(--color-text);
  }
  .option:hover { border-color: var(--color-primary); }
  .option.selected { border-color: var(--color-primary); background: #1e1b4b; }
  .option h3 { font-size: 1rem; margin-bottom: 0.25rem; display: flex; align-items: center; gap: 0.5rem; }
  .badge { font-size: 0.65rem; background: #4ade80; color: #000; padding: 0.15rem 0.5rem; border-radius: 99px; font-weight: 600; }
  .option p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; margin-bottom: 0.5rem; }
  .preview { display: inline-block; font-size: 0.8rem; background: #0d0d0d; padding: 0.25rem 0.5rem; border-radius: 4px; margin-bottom: 0.5rem; }
  .tag { font-size: 0.7rem; color: var(--color-text-muted); background: var(--color-border); padding: 0.15rem 0.5rem; border-radius: 4px; }
</style>
```

**Step 2: Wire in, build, commit**

```bash
npm run build
git add -A
git commit -m "feat: add template language step with syntax previews"
```

---

## Task 6: Styling Step

**Files:**
- Create: `src/lib/components/wizard/Styling.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

Same card pattern as TemplateLang but for CSS options: Vanilla CSS (recommended), Tailwind, Sass. Teaches what CSS is and why different tools exist.

**Step 1: Create component, wire in, build, commit**

```bash
git commit -m "feat: add styling step with CSS approach explanations"
```

---

## Task 7: IndieWeb Step

**Files:**
- Create: `src/lib/components/wizard/IndieWeb.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

**Step 1: Create IndieWeb component**

`src/lib/components/wizard/IndieWeb.svelte`:
```svelte
<script lang="ts">
  import { wizard, updateIndieweb } from '$lib/stores/wizard';
</script>

<div class="illustration">&#127760;</div>
<h1>Welcome to the IndieWeb</h1>
<p class="explanation">
  <strong>The IndieWeb is a movement to take back your content from big platforms.</strong>
  Instead of posting on Twitter and hoping it stays there, you publish on your own
  site first, then optionally share to social media. You're in control.
</p>

<div class="principles">
  <div class="principle">
    <strong>&#127968; Own your data</strong>
    <p>Your posts, photos, and thoughts live on YOUR domain. No platform can delete them or change the rules.</p>
  </div>
  <div class="principle">
    <strong>&#128257; POSSE</strong>
    <p>Publish on your Own Site, Syndicate Elsewhere. Post on your blog first, then share copies to social media.</p>
  </div>
  <div class="principle">
    <strong>&#128279; Connected but independent</strong>
    <p>IndieWeb sites can talk to each other — replies, likes, and mentions work across different websites.</p>
  </div>
</div>

<h2>Enable IndieWeb features</h2>
<p class="explanation">
  These add special tags to your site that enable cross-site communication.
  They're just placeholders for now — we'll set up the real endpoints later.
</p>

<div class="toggles">
  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.webmention}
      onchange={(e) => updateIndieweb({ webmention: e.currentTarget.checked })}>
    <div>
      <strong>Webmention</strong>
      <p>Like @mentions but across websites. When someone replies to your post from their blog, you'll know about it.</p>
    </div>
  </label>

  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.micropub}
      onchange={(e) => updateIndieweb({ micropub: e.currentTarget.checked })}>
    <div>
      <strong>Micropub</strong>
      <p>Post to your site from any app — not just Twelvety. It's like having an API for your blog.</p>
    </div>
  </label>

  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.indieauth}
      onchange={(e) => updateIndieweb({ indieauth: e.currentTarget.checked })}>
    <div>
      <strong>IndieAuth</strong>
      <p>Sign in to other websites using your own domain. Your URL is your username.</p>
    </div>
  </label>
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  h2 { font-size: 1.25rem; margin: 1.5rem 0 0.5rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1rem; }
  .explanation strong { color: var(--color-text); }
  .principles { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 1.5rem; }
  .principle {
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem;
  }
  .principle strong { display: block; margin-bottom: 0.25rem; }
  .principle p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; }
  .toggles { display: flex; flex-direction: column; gap: 0.75rem; }
  .toggle {
    display: flex; gap: 0.75rem; align-items: flex-start;
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem; cursor: pointer;
  }
  .toggle input { margin-top: 0.25rem; flex-shrink: 0; width: 18px; height: 18px; }
  .toggle strong { display: block; margin-bottom: 0.15rem; }
  .toggle p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; }
</style>
```

**Step 2: Wire in, build, commit**

```bash
git commit -m "feat: add indieweb step with principles and protocol toggles"
```

---

## Task 8: Review & Create Step

**Files:**
- Create: `src/lib/components/wizard/Review.svelte`
- Modify: `src/routes/new/wizard/+page.svelte`

**Step 1: Create Review component**

`src/lib/components/wizard/Review.svelte`:
```svelte
<script lang="ts">
  import { wizard } from '$lib/stores/wizard';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import type { ProjectEntry, ScaffoldOptions } from '$lib/types';

  let creating = $state(false);
  let errorMsg = $state('');

  async function createSite() {
    const w = $wizard;
    if (!w.identity.name || !w.identity.url || !w.site.name || !w.site.directory) {
      errorMsg = 'Please go back and fill in all required fields.';
      return;
    }
    creating = true;
    errorMsg = '';
    try {
      const options: ScaffoldOptions = {
        name: w.site.name,
        directory: `${w.site.directory}/${w.site.name.toLowerCase().replace(/\s+/g, '-')}`,
        starter: 'blog',
        template_lang: w.templateLang,
        css: w.css,
        author_name: w.identity.name,
        author_url: w.identity.url,
        site_url: w.identity.url,
      };
      const project = await invoke<ProjectEntry>('scaffold_project', { options });
      goto(`/project/${project.id}`);
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to create site';
    } finally {
      creating = false;
    }
  }
</script>

<div class="illustration">&#127881;</div>
<h1>Ready to create your site!</h1>
<p class="explanation">Here's a summary of your choices. You can go back to change anything.</p>

<div class="summary">
  <div class="row">
    <span class="label">Your name</span>
    <span class="value">{$wizard.identity.name || '—'}</span>
  </div>
  <div class="row">
    <span class="label">Your URL</span>
    <span class="value">{$wizard.identity.url || '—'}</span>
  </div>
  <div class="row">
    <span class="label">Site name</span>
    <span class="value">{$wizard.site.name || '—'}</span>
  </div>
  <div class="row">
    <span class="label">Location</span>
    <span class="value mono">{$wizard.site.directory || '—'}</span>
  </div>
  <div class="row">
    <span class="label">Template language</span>
    <span class="value">{$wizard.templateLang}</span>
  </div>
  <div class="row">
    <span class="label">CSS approach</span>
    <span class="value">{$wizard.css}</span>
  </div>
  <div class="row">
    <span class="label">IndieWeb</span>
    <span class="value">
      {[
        $wizard.indieweb.webmention && 'Webmention',
        $wizard.indieweb.micropub && 'Micropub',
        $wizard.indieweb.indieauth && 'IndieAuth',
      ].filter(Boolean).join(', ') || 'None'}
    </span>
  </div>
</div>

{#if errorMsg}
  <p class="error">{errorMsg}</p>
{/if}

<button class="btn-create" onclick={createSite} disabled={creating}>
  {creating ? 'Creating your site...' : 'Create Site'}
</button>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); margin-bottom: 1.5rem; }
  .summary {
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem; margin-bottom: 1.5rem;
  }
  .row { display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--color-border); }
  .row:last-child { border-bottom: none; }
  .label { color: var(--color-text-muted); font-size: 0.875rem; }
  .value { font-weight: 500; font-size: 0.875rem; }
  .mono { font-family: monospace; font-size: 0.75rem; }
  .error { color: var(--color-danger); font-size: 0.875rem; margin-bottom: 1rem; }
  .btn-create {
    width: 100%; padding: 0.875rem; background: var(--color-primary); color: white;
    border: none; border-radius: var(--radius); font-size: 1rem; font-weight: 600; cursor: pointer;
  }
  .btn-create:hover { background: var(--color-primary-hover); }
  .btn-create:disabled { opacity: 0.6; cursor: not-allowed; }
</style>
```

**Step 2: Wire in all steps to wizard page**

Final `src/routes/new/wizard/+page.svelte`:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { wizard, resetWizard } from '$lib/stores/wizard';
  import WizardShell from '$lib/components/wizard/WizardShell.svelte';
  import Welcome from '$lib/components/wizard/Welcome.svelte';
  import Identity from '$lib/components/wizard/Identity.svelte';
  import SiteBasics from '$lib/components/wizard/SiteBasics.svelte';
  import TemplateLang from '$lib/components/wizard/TemplateLang.svelte';
  import Styling from '$lib/components/wizard/Styling.svelte';
  import IndieWeb from '$lib/components/wizard/IndieWeb.svelte';
  import Review from '$lib/components/wizard/Review.svelte';

  onMount(() => resetWizard());
</script>

<WizardShell>
  {#if $wizard.step === 0}
    <Welcome />
  {:else if $wizard.step === 1}
    <Identity />
  {:else if $wizard.step === 2}
    <SiteBasics />
  {:else if $wizard.step === 3}
    <TemplateLang />
  {:else if $wizard.step === 4}
    <Styling />
  {:else if $wizard.step === 5}
    <IndieWeb />
  {:else if $wizard.step === 6}
    <Review />
  {/if}
</WizardShell>
```

**Step 3: Build, commit**

```bash
npm run build
git add -A
git commit -m "feat: add review step and wire all wizard steps together"
```

---

## Task 9: Import Wizard — Rust Detection Command

**Files:**
- Create: `src-tauri/src/commands/import.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Create the detection command with tests**

`src-tauri/src/commands/import.rs`:
```rust
use crate::TwelvetyError;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct DetectionResult {
    pub is_eleventy: bool,
    pub config_file: Option<String>,
    pub template_lang: Option<String>,
    pub post_count: usize,
    pub has_indieweb_markup: bool,
    pub has_twelvety_config: bool,
    pub has_feeds: bool,
}

fn detect_template_lang(dir: &Path) -> Option<String> {
    let extensions = [
        ("njk", "nunjucks"),
        ("liquid", "liquid"),
        ("webc", "webc"),
    ];
    for (ext, lang) in &extensions {
        let pattern = format!("*.{}", ext);
        if has_files_with_ext(dir, ext) {
            return Some(lang.to_string());
        }
    }
    None
}

fn has_files_with_ext(dir: &Path, ext: &str) -> bool {
    fn walk(dir: &Path, ext: &str) -> bool {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if walk(&path, ext) { return true; }
                } else if path.extension().is_some_and(|e| e == ext) {
                    return true;
                }
            }
        }
        false
    }
    walk(dir, ext)
}

fn count_markdown_files(dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, count);
                } else if path.extension().is_some_and(|e| e == "md") {
                    *count += 1;
                }
            }
        }
    }
    let mut count = 0;
    walk(dir, &mut count);
    count
}

fn check_indieweb_markup(dir: &Path) -> bool {
    fn walk(dir: &Path) -> bool {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if walk(&path) { return true; }
                } else if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.contains("h-card") || content.contains("h-entry") || content.contains("h-feed") {
                        return true;
                    }
                }
            }
        }
        false
    }
    walk(dir)
}

pub fn detect_project(dir: &Path) -> DetectionResult {
    let config_candidates = [
        "eleventy.config.js",
        "eleventy.config.mjs",
        "eleventy.config.cjs",
        ".eleventy.js",
    ];
    let config_file = config_candidates
        .iter()
        .find(|f| dir.join(f).exists())
        .map(|f| f.to_string());

    let is_eleventy = config_file.is_some()
        || dir.join("package.json").exists()
            && std::fs::read_to_string(dir.join("package.json"))
                .unwrap_or_default()
                .contains("@11ty/eleventy");

    let has_twelvety = crate::config::find_config_file(dir).is_some();
    let has_feeds = dir.join("_site/feed.xml").exists()
        || dir.join("_site/feed.json").exists()
        || has_files_with_ext(dir, "xml");

    DetectionResult {
        is_eleventy,
        config_file,
        template_lang: detect_template_lang(dir),
        post_count: count_markdown_files(dir),
        has_indieweb_markup: check_indieweb_markup(dir),
        has_twelvety_config: has_twelvety,
        has_feeds,
    }
}

#[tauri::command]
pub async fn detect_eleventy_project(directory: String) -> Result<DetectionResult, TwelvetyError> {
    let path = PathBuf::from(&directory);
    if !path.exists() {
        return Err(TwelvetyError {
            code: "DIR_NOT_FOUND".to_string(),
            message: format!("Directory not found: {}", directory),
        });
    }
    Ok(detect_project(&path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect_project(dir.path());
        assert!(!result.is_eleventy);
        assert_eq!(result.post_count, 0);
    }

    #[test]
    fn test_detect_eleventy_by_config() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("eleventy.config.js"), "module.exports = {}").unwrap();
        let result = detect_project(dir.path());
        assert!(result.is_eleventy);
        assert_eq!(result.config_file, Some("eleventy.config.js".to_string()));
    }

    #[test]
    fn test_detect_template_lang_and_posts() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("eleventy.config.js"), "").unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/index.njk"), "").unwrap();
        std::fs::write(dir.path().join("src/post.md"), "# Hello").unwrap();
        let result = detect_project(dir.path());
        assert_eq!(result.template_lang, Some("nunjucks".to_string()));
        assert_eq!(result.post_count, 1);
    }

    #[test]
    fn test_detect_indieweb_markup() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("layout.njk"), "<div class=\"h-card\">").unwrap();
        let result = detect_project(dir.path());
        assert!(result.has_indieweb_markup);
    }
}
```

**Step 2: Register command**

Add `pub mod import;` to `commands/mod.rs`.
Add `commands::import::detect_eleventy_project,` to invoke_handler in `lib.rs`.

**Step 3: Run tests, commit**

```bash
cd src-tauri && cargo test && cargo fmt
cd .. && git add -A
git commit -m "feat: add detect_eleventy_project command for import wizard"
```

---

## Task 10: Import Wizard — Frontend

**Files:**
- Create: `src/routes/new/import/+page.svelte`

**Step 1: Create the import flow page**

Four stages: pick directory, detecting, report, register. Uses `detect_eleventy_project` and `add_project` IPC commands.

Shows detection results with checkboxes for optional enhancements (add twelvety.config.js, add indieweb markup, add feeds).

**Step 2: Build, commit**

```bash
git commit -m "feat: add import wizard for existing Eleventy sites"
```

---

## Task 11: Advanced Create

**Files:**
- Create: `src/routes/new/advanced/+page.svelte`

**Step 1: Move current new project form to advanced route**

Copy the existing `src/routes/new/+page.svelte` form content (before the redirect change) into `advanced/+page.svelte`. Add a "Back to wizard" link.

**Step 2: Build, commit**

```bash
git commit -m "feat: add advanced quick-create form"
```

---

## Task 12: Update Dashboard with Three Entry Points

**Files:**
- Modify: `src/routes/+page.svelte`

**Step 1: Update dashboard**

Replace single "New Project" button with three action cards matching the Welcome step: "Create Your Site" (links to /new/wizard), "Open Existing Site" (links to /new/import), "Advanced Create" (links to /new/advanced).

**Step 2: Build, commit**

```bash
git commit -m "feat: update dashboard with three project creation entry points"
```

---

## Task Summary

| Task | What | Files |
|------|------|-------|
| 1 | Wizard state store + tests | stores/wizard.ts |
| 2 | Wizard shell + welcome step | WizardShell.svelte, Welcome.svelte |
| 3 | Identity step (h-card teaching) | Identity.svelte |
| 4 | Site basics step | SiteBasics.svelte |
| 5 | Template language step | TemplateLang.svelte |
| 6 | Styling step | Styling.svelte |
| 7 | IndieWeb step (full teaching) | IndieWeb.svelte |
| 8 | Review & create step | Review.svelte |
| 9 | Import detection (Rust) | commands/import.rs |
| 10 | Import wizard (frontend) | import/+page.svelte |
| 11 | Advanced create form | advanced/+page.svelte |
| 12 | Dashboard update | +page.svelte |
