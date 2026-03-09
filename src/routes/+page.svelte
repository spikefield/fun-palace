<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { projects, loadProjects } from '$lib/stores/projects';
  import { wizard, resetWizard, updateIdentity, updateSite, updateIndieweb } from '$lib/stores/wizard';
  import type { ProjectEntry, ScaffoldOptions } from '$lib/types';

  type Flow = 'idle' | 'create' | 'import';

  let ready = $state(false);
  let flow = $state<Flow>('idle');
  let createStep = $state(0);

  // Import flow state
  interface DetectionResult {
    is_eleventy: boolean;
    config_file: string | null;
    template_lang: string | null;
    post_count: number;
    has_indieweb_markup: boolean;
    has_funpalace_config: boolean;
    has_feeds: boolean;
  }
  let importDir = $state('');
  let importStage = $state<'idle' | 'detecting' | 'report' | 'error'>('idle');
  let importResult = $state<DetectionResult | null>(null);
  let importError = $state('');
  let registering = $state(false);

  // Create flow state
  let creating = $state(false);
  let createError = $state('');

  const templateOptions = [
    { value: 'webc', name: 'WebC', recommended: true },
    { value: 'nunjucks', name: 'Nunjucks' },
    { value: 'liquid', name: 'Liquid' },
    { value: 'jsx', name: 'JSX' },
    { value: 'mdx', name: 'MDX' },
    { value: 'typescript', name: 'TypeScript' },
    { value: 'handlebars', name: 'Handlebars' },
    { value: 'pug', name: 'Pug' },
    { value: 'mustache', name: 'Mustache' },
    { value: 'ejs', name: 'EJS' },
    { value: 'haml', name: 'HAML' },
  ];

  const cssOptions = [
    { value: 'vanilla', name: 'Vanilla CSS', recommended: true },
    { value: 'tailwind', name: 'Tailwind' },
    { value: 'sass', name: 'Sass' },
  ];

  onMount(async () => {
    await loadProjects();
    ready = true;
  });

  function startCreate() {
    resetWizard();
    flow = 'create';
    createStep = 0;
  }

  async function startImport() {
    flow = 'import';
    importStage = 'idle';
    await tick();
    pickImportDir();
  }

  async function pickImportDir() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      importDir = selected as string;
      importStage = 'detecting';
      importError = '';
      try {
        importResult = await invoke<DetectionResult>('detect_eleventy_project', { directory: importDir });
        importStage = 'report';
      } catch (e: unknown) {
        const err = e as { message?: string };
        importError = err.message ?? 'Detection failed';
        importStage = 'error';
      }
    } else {
      // User cancelled — go back to idle
      flow = 'idle';
    }
  }

  async function registerImport() {
    if (!importResult || !importDir) return;
    registering = true;
    try {
      const name = importDir.split('/').pop() || 'Imported Site';
      await invoke('add_project', {
        name,
        path: importDir,
        templateLang: importResult.template_lang || 'nunjucks',
        cssApproach: 'vanilla',
      });
      await loadProjects();
      flow = 'idle';
      importStage = 'idle';
    } catch (e: unknown) {
      const err = e as { message?: string };
      importError = err.message ?? 'Failed to register project';
    } finally {
      registering = false;
    }
  }

  async function advanceCreate() {
    createStep++;
    await tick();
    scrollToBottom();
  }

  async function pickDirectory() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      updateSite({ directory: selected as string });
    }
  }

  function selectTemplate(value: string) {
    wizard.update((s) => ({ ...s, templateLang: value }));
  }

  function selectCss(value: string) {
    wizard.update((s) => ({ ...s, css: value }));
  }

  async function createSite() {
    const w = $wizard;
    creating = true;
    createError = '';
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
      createError = err.message ?? 'Failed to create site';
    } finally {
      creating = false;
    }
  }

  function scrollToBottom() {
    setTimeout(() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' }), 50);
  }

  function startOver() {
    flow = 'idle';
    createStep = 0;
    resetWizard();
  }

  // Validation helpers
  let canContinueIdentity = $derived(
    $wizard.identity.name.trim() !== '' && $wizard.identity.url.trim() !== ''
  );
  let canContinueSite = $derived(
    $wizard.site.name.trim() !== '' && $wizard.site.directory.trim() !== ''
  );
</script>

<div class="thread">
  <!-- Turn 1: Welcome -->
  <div class="palace-turn">
    <p class="greeting">Welcome to <strong>Fun Palace</strong>.</p>
    <p class="body">Your sites, your web. What would you like to do?</p>
  </div>

  {#if ready}
    {#if flow === 'idle'}
      <div class="choices">
        <button class="choice" onclick={startCreate}>
          Create a new site
        </button>

        {#if $projects.length > 0}
          {#each $projects as project}
            <button class="choice" onclick={() => goto(`/project/${project.id}`)}>
              Work on {project.name}
            </button>
          {/each}
        {/if}

        <button class="choice choice-muted" onclick={startImport}>
          Import an existing site
        </button>
      </div>

    {:else if flow === 'import'}
      {#if importStage === 'detecting'}
        <div class="palace-turn">
          <p class="body">Scanning {importDir}...</p>
          <div class="spinner"></div>
        </div>
      {:else if importStage === 'report' && importResult}
        <div class="palace-turn">
          {#if !importResult.is_eleventy}
            <p class="body">This doesn't look like an Eleventy project — no config file or @11ty/eleventy dependency was found. You can still add it, but it may not work as expected.</p>
          {:else}
            <p class="body">Found an Eleventy project.</p>
          {/if}

          <div class="summary">
            <div class="row"><span class="label">Location</span><span class="value mono">{importDir}</span></div>
            <div class="row"><span class="label">Template language</span><span class="value">{importResult.template_lang || 'Not detected'}</span></div>
            <div class="row"><span class="label">Content files</span><span class="value">{importResult.post_count} markdown files</span></div>
            <div class="row"><span class="label">IndieWeb markup</span><span class="value">{importResult.has_indieweb_markup ? 'Found' : 'Not found'}</span></div>
            <div class="row"><span class="label">Feeds</span><span class="value">{importResult.has_feeds ? 'Found' : 'Not found'}</span></div>
          </div>

          <div class="choices">
            <button class="choice" onclick={registerImport} disabled={registering}>
              {registering ? 'Adding...' : 'Add to Fun Palace'}
            </button>
            <button class="choice choice-muted" onclick={pickImportDir}>Choose different folder</button>
            <button class="choice choice-muted" onclick={startOver}>Start over</button>
          </div>
        </div>
      {:else if importStage === 'error'}
        <div class="palace-turn">
          <p class="body error">{importError}</p>
          <div class="choices">
            <button class="choice" onclick={pickImportDir}>Try again</button>
            <button class="choice choice-muted" onclick={startOver}>Start over</button>
          </div>
        </div>
      {/if}

    {:else if flow === 'create'}
      <!-- Step 0: Identity -->
      {#if createStep >= 0}
        {#if createStep > 0}
          <!-- Completed summary -->
          <div class="palace-turn done"><p class="body">Who are you on the web?</p></div>
          <div class="user-turn">{$wizard.identity.name} · {$wizard.identity.url}</div>
        {:else}
          <!-- Active -->
          <div class="palace-turn">
            <p class="body">First, who are you on the web? On the indieweb, your identity lives on your own domain.</p>
            <div class="fields">
              <label>
                <span class="field-label">Your Name</span>
                <input type="text" value={$wizard.identity.name}
                  oninput={(e) => updateIdentity({ name: e.currentTarget.value })}
                  placeholder="Alice Example">
              </label>
              <label>
                <span class="field-label">Your Website URL</span>
                <input type="url" value={$wizard.identity.url}
                  oninput={(e) => updateIdentity({ url: e.currentTarget.value })}
                  placeholder="https://alice.example.com">
              </label>
              <label>
                <span class="field-label">Email <span class="optional">optional</span></span>
                <input type="email" value={$wizard.identity.email}
                  oninput={(e) => updateIdentity({ email: e.currentTarget.value })}
                  placeholder="alice@example.com">
              </label>
            </div>
            <div class="choices">
              <button class="choice" onclick={advanceCreate} disabled={!canContinueIdentity}>Continue</button>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Step 1: Site basics -->
      {#if createStep >= 1}
        {#if createStep > 1}
          <div class="palace-turn done"><p class="body">What should we call your site?</p></div>
          <div class="user-turn">{$wizard.site.name}</div>
        {:else}
          <div class="palace-turn">
            <p class="body">Nice to meet you, {$wizard.identity.name}. What should we call your site?</p>
            <div class="fields">
              <label>
                <span class="field-label">Site Name</span>
                <input type="text" value={$wizard.site.name}
                  oninput={(e) => updateSite({ name: e.currentTarget.value })}
                  placeholder="Alice's Garden">
              </label>
              <label>
                <span class="field-label">Save Location</span>
                <div class="dir-picker">
                  <input type="text" value={$wizard.site.directory} placeholder="Choose a folder..." readonly>
                  <button class="choice" onclick={pickDirectory}>Browse</button>
                </div>
              </label>
            </div>
            <div class="choices">
              <button class="choice" onclick={advanceCreate} disabled={!canContinueSite}>Continue</button>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Step 2: Template language -->
      {#if createStep >= 2}
        {#if createStep > 2}
          <div class="palace-turn done"><p class="body">How should your templates work?</p></div>
          <div class="user-turn">{templateOptions.find(t => t.value === $wizard.templateLang)?.name ?? $wizard.templateLang}</div>
        {:else}
          <div class="palace-turn">
            <p class="body">How should your templates work? Templates control how your content turns into web pages — like mail merge for the web.</p>
            <div class="choices">
              {#each templateOptions as opt}
                <button
                  class="choice"
                  class:selected={$wizard.templateLang === opt.value}
                  onclick={() => selectTemplate(opt.value)}
                >
                  {opt.name}{#if opt.recommended} ·  recommended{/if}
                </button>
              {/each}
            </div>
            <div class="choices" style="margin-top: 0.75rem;">
              <button class="choice" onclick={advanceCreate}>Continue with {templateOptions.find(t => t.value === $wizard.templateLang)?.name}</button>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Step 3: CSS -->
      {#if createStep >= 3}
        {#if createStep > 3}
          <div class="palace-turn done"><p class="body">How do you want to style it?</p></div>
          <div class="user-turn">{cssOptions.find(c => c.value === $wizard.css)?.name ?? $wizard.css}</div>
        {:else}
          <div class="palace-turn">
            <p class="body">And how do you want to style your site?</p>
            <div class="choices">
              {#each cssOptions as opt}
                <button
                  class="choice"
                  class:selected={$wizard.css === opt.value}
                  onclick={() => selectCss(opt.value)}
                >
                  {opt.name}{#if opt.recommended} · recommended{/if}
                </button>
              {/each}
            </div>
            <div class="choices" style="margin-top: 0.75rem;">
              <button class="choice" onclick={advanceCreate}>Continue with {cssOptions.find(c => c.value === $wizard.css)?.name}</button>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Step 4: IndieWeb -->
      {#if createStep >= 4}
        {#if createStep > 4}
          <div class="palace-turn done"><p class="body">IndieWeb features?</p></div>
          <div class="user-turn">
            {[
              $wizard.indieweb.webmention && 'Webmention',
              $wizard.indieweb.micropub && 'Micropub',
              $wizard.indieweb.indieauth && 'IndieAuth',
            ].filter(Boolean).join(', ') || 'None'}
          </div>
        {:else}
          <div class="palace-turn">
            <p class="body">Almost there. The IndieWeb lets your site talk to others — replies, likes, and mentions work across different websites.</p>
            <div class="toggles">
              <label class="toggle">
                <input type="checkbox" checked={$wizard.indieweb.webmention}
                  onchange={(e) => updateIndieweb({ webmention: e.currentTarget.checked })}>
                <div>
                  <strong>Webmention</strong>
                  <p>Cross-site @mentions — know when someone links to your post</p>
                </div>
              </label>
              <label class="toggle">
                <input type="checkbox" checked={$wizard.indieweb.micropub}
                  onchange={(e) => updateIndieweb({ micropub: e.currentTarget.checked })}>
                <div>
                  <strong>Micropub</strong>
                  <p>Post to your site from any app, not just Fun Palace</p>
                </div>
              </label>
              <label class="toggle">
                <input type="checkbox" checked={$wizard.indieweb.indieauth}
                  onchange={(e) => updateIndieweb({ indieauth: e.currentTarget.checked })}>
                <div>
                  <strong>IndieAuth</strong>
                  <p>Sign in to other sites using your own domain</p>
                </div>
              </label>
            </div>
            <div class="choices" style="margin-top: 0.75rem;">
              <button class="choice" onclick={advanceCreate}>Continue</button>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Step 5: Review & Create -->
      {#if createStep >= 5}
        <div class="palace-turn">
          <p class="body">Here's what we're building:</p>
          <div class="summary">
            <div class="row"><span class="label">Your name</span><span class="value">{$wizard.identity.name}</span></div>
            <div class="row"><span class="label">Your URL</span><span class="value">{$wizard.identity.url}</span></div>
            <div class="row"><span class="label">Site name</span><span class="value">{$wizard.site.name}</span></div>
            <div class="row"><span class="label">Location</span><span class="value mono">{$wizard.site.directory}</span></div>
            <div class="row"><span class="label">Templates</span><span class="value">{templateOptions.find(t => t.value === $wizard.templateLang)?.name}</span></div>
            <div class="row"><span class="label">Styling</span><span class="value">{cssOptions.find(c => c.value === $wizard.css)?.name}</span></div>
            <div class="row">
              <span class="label">IndieWeb</span>
              <span class="value">{[
                $wizard.indieweb.webmention && 'Webmention',
                $wizard.indieweb.micropub && 'Micropub',
                $wizard.indieweb.indieauth && 'IndieAuth',
              ].filter(Boolean).join(', ') || 'None'}</span>
            </div>
          </div>

          {#if createError}
            <p class="body error">{createError}</p>
          {/if}

          <div class="choices">
            <button class="choice" onclick={createSite} disabled={creating}>
              {creating ? 'Creating...' : 'Create site'}
            </button>
            <button class="choice choice-muted" onclick={startOver}>Start over</button>
          </div>
        </div>
      {/if}
    {/if}
  {/if}
</div>

<style>
  .thread {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding-top: 15vh;
    padding-bottom: 4rem;
  }

  /* Palace turns — left-aligned, monospace */
  .palace-turn {
    font-family: var(--font-palace);
  }
  .palace-turn.done {
    opacity: 0.5;
  }

  .greeting {
    font-size: 1.25rem;
    margin-bottom: 0.75rem;
    color: var(--color-text);
  }
  .greeting strong {
    font-weight: 600;
  }

  .body {
    color: var(--color-text-muted);
    line-height: 1.8;
    font-size: 0.85rem;
  }

  /* User turns — right-aligned, sans-serif */
  .user-turn {
    font-family: var(--font-user);
    align-self: flex-end;
    background: var(--color-primary);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 999px;
    font-size: 0.8rem;
    font-weight: 500;
    max-width: 80%;
  }

  /* Choices — pill buttons */
  .choices {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.75rem;
  }

  .choice {
    font-family: var(--font-user);
    padding: 0.5rem 1rem;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: 999px;
    color: var(--color-text);
    font-weight: 500;
    font-size: 0.8rem;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
  }
  .choice:hover:not(:disabled) {
    border-color: var(--color-primary);
    background: var(--color-surface);
  }
  .choice:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .choice.selected {
    border-color: var(--color-primary);
    background: var(--color-primary);
    color: white;
  }
  .choice-muted {
    color: var(--color-text-muted);
  }

  /* Form fields */
  .fields {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.75rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .field-label {
    font-family: var(--font-palace);
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }
  .optional {
    opacity: 0.6;
  }

  input {
    font-family: var(--font-user);
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    font-size: 0.85rem;
    background: var(--color-surface);
    color: var(--color-text);
  }
  input:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .dir-picker {
    display: flex;
    gap: 0.5rem;
  }
  .dir-picker input {
    flex: 1;
  }

  /* Toggles */
  .toggles {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.75rem;
  }
  .toggle {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    cursor: pointer;
    font-family: var(--font-user);
  }
  .toggle input[type="checkbox"] {
    margin-top: 0.15rem;
    flex-shrink: 0;
    width: 16px;
    height: 16px;
  }
  .toggle strong {
    font-size: 0.85rem;
    display: block;
    color: var(--color-text);
  }
  .toggle p {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    line-height: 1.4;
  }

  /* Summary table */
  .summary {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 0.75rem;
    margin-top: 0.75rem;
    font-family: var(--font-user);
  }
  .row {
    display: flex;
    justify-content: space-between;
    padding: 0.35rem 0;
    border-bottom: 1px solid var(--color-border);
  }
  .row:last-child { border-bottom: none; }
  .label { color: var(--color-text-muted); font-size: 0.8rem; }
  .value { font-weight: 500; font-size: 0.8rem; }
  .mono { font-family: var(--font-palace); font-size: 0.75rem; }

  /* Error */
  .error { color: var(--color-danger); }

  /* Spinner */
  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-top: 0.5rem;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
