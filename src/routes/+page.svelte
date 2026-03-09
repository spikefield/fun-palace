<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { projects, loadProjects } from '$lib/stores/projects';
  import { wizard, resetWizard, updateIdentity, updateSite, updateIndieweb } from '$lib/stores/wizard';
  import { Thread, PalaceTurn, Choices, Choice, Fields, Summary } from '$lib/components/conversation';
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
      flow = 'idle';
    }
  }

  async function registerImport() {
    if (!importResult || !importDir) return;
    registering = true;
    try {
      const name = importDir.split('/').pop() || 'Imported Site';
      const entry: ProjectEntry = {
        id: crypto.randomUUID(),
        name,
        path: importDir,
        created_at: new Date().toISOString(),
        template_lang: importResult.template_lang || 'nunjucks',
        css_approach: 'vanilla',
      };
      await invoke('add_project', { entry });
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
    setTimeout(() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' }), 50);
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

  function startOver() {
    flow = 'idle';
    createStep = 0;
    resetWizard();
  }

  let canContinueIdentity = $derived(
    $wizard.identity.name.trim() !== '' && $wizard.identity.url.trim() !== ''
  );
  let canContinueSite = $derived(
    $wizard.site.name.trim() !== '' && $wizard.site.directory.trim() !== ''
  );
</script>

<Thread>
  <PalaceTurn>
    <p class="greeting">Welcome to <strong>Fun Palace</strong>.</p>
    <p>Your sites, your web. What would you like to do?</p>
  </PalaceTurn>

  {#if ready}
    {#if flow === 'idle'}
      <Choices>
        <Choice onclick={startCreate}>Create a new site</Choice>
        <Choice muted onclick={startImport}>Import an existing site</Choice>
      </Choices>

      {#if $projects.length > 0}
        <PalaceTurn>
          <p>Or pick up where you left off:</p>
        </PalaceTurn>
        <div class="site-list">
          {#each $projects as project}
            <button class="site-item" onclick={() => goto(`/project/${project.id}`)}>
              {project.name}
            </button>
          {/each}
        </div>
      {/if}

    {:else if flow === 'import'}
      {#if importStage === 'detecting'}
        <PalaceTurn>
          <p>Scanning {importDir}...</p>
          <div class="spinner"></div>
        </PalaceTurn>
      {:else if importStage === 'report' && importResult}
        <PalaceTurn>
          {#if !importResult.is_eleventy}
            <p>This doesn't look like an Eleventy project — no config file or @11ty/eleventy dependency was found. You can still add it, but it may not work as expected.</p>
          {:else}
            <p>Found an Eleventy project.</p>
          {/if}

          <Summary rows={[
            { label: 'Location', value: importDir, mono: true },
            { label: 'Template language', value: importResult.template_lang || 'Not detected' },
            { label: 'Markdown files', value: String(importResult.post_count) },
            { label: 'IndieWeb markup', value: importResult.has_indieweb_markup ? 'Found' : 'Not found' },
            { label: 'Feeds', value: importResult.has_feeds ? 'Found' : 'Not found' },
          ]} />

          <Choices>
            <Choice onclick={registerImport} disabled={registering}>
              {registering ? 'Adding...' : 'Add to Fun Palace'}
            </Choice>
            <Choice muted onclick={pickImportDir}>Choose different folder</Choice>
            <Choice muted onclick={startOver}>Start over</Choice>
          </Choices>
        </PalaceTurn>
      {:else if importStage === 'error'}
        <PalaceTurn>
          <p class="error">{importError}</p>
          <Choices>
            <Choice onclick={pickImportDir}>Try again</Choice>
            <Choice muted onclick={startOver}>Start over</Choice>
          </Choices>
        </PalaceTurn>
      {/if}

    {:else if flow === 'create'}
      <!-- Step 0: Identity -->
      <PalaceTurn>
        <p>First, who are you on the web? On the indieweb, your identity lives on your own domain.</p>
        <Fields>
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
        </Fields>
        {#if createStep === 0}
          <Choices>
            <Choice onclick={advanceCreate} disabled={!canContinueIdentity}>Continue</Choice>
          </Choices>
        {/if}
      </PalaceTurn>

      <!-- Step 1: Site basics -->
      {#if createStep >= 1}
        <PalaceTurn>
          <p>Nice to meet you, {$wizard.identity.name}. What should we call your site?</p>
          <Fields>
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
                <Choice onclick={pickDirectory}>Browse</Choice>
              </div>
            </label>
          </Fields>
          {#if createStep === 1}
            <Choices>
              <Choice onclick={advanceCreate} disabled={!canContinueSite}>Continue</Choice>
            </Choices>
          {/if}
        </PalaceTurn>
      {/if}

      <!-- Step 2: Template language -->
      {#if createStep >= 2}
        <PalaceTurn>
          <p>How should your templates work? Templates control how your content turns into web pages — like mail merge for the web.</p>
          <Choices>
            {#each templateOptions as opt}
              <Choice
                selected={$wizard.templateLang === opt.value}
                onclick={() => selectTemplate(opt.value)}
              >
                {opt.name}{#if opt.recommended} · recommended{/if}
              </Choice>
            {/each}
          </Choices>
          {#if createStep === 2}
            <Choices>
              <Choice onclick={advanceCreate}>Continue with {templateOptions.find(t => t.value === $wizard.templateLang)?.name}</Choice>
            </Choices>
          {/if}
        </PalaceTurn>
      {/if}

      <!-- Step 3: CSS -->
      {#if createStep >= 3}
        <PalaceTurn>
          <p>And how do you want to style your site?</p>
          <Choices>
            {#each cssOptions as opt}
              <Choice
                selected={$wizard.css === opt.value}
                onclick={() => selectCss(opt.value)}
              >
                {opt.name}{#if opt.recommended} · recommended{/if}
              </Choice>
            {/each}
          </Choices>
          {#if createStep === 3}
            <Choices>
              <Choice onclick={advanceCreate}>Continue with {cssOptions.find(c => c.value === $wizard.css)?.name}</Choice>
            </Choices>
          {/if}
        </PalaceTurn>
      {/if}

      <!-- Step 4: IndieWeb -->
      {#if createStep >= 4}
        <PalaceTurn>
          <p>Almost there. The IndieWeb lets your site talk to others — replies, likes, and mentions work across different websites.</p>
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
          {#if createStep === 4}
            <Choices>
              <Choice onclick={advanceCreate}>Continue</Choice>
            </Choices>
          {/if}
        </PalaceTurn>
      {/if}

      <!-- Step 5: Review & Create -->
      {#if createStep >= 5}
        <PalaceTurn>
          <p>Here's what we're building:</p>
          <Summary rows={[
            { label: 'Your name', value: $wizard.identity.name },
            { label: 'Your URL', value: $wizard.identity.url },
            { label: 'Site name', value: $wizard.site.name },
            { label: 'Location', value: $wizard.site.directory, mono: true },
            { label: 'Templates', value: templateOptions.find(t => t.value === $wizard.templateLang)?.name ?? $wizard.templateLang },
            { label: 'Styling', value: cssOptions.find(c => c.value === $wizard.css)?.name ?? $wizard.css },
            { label: 'IndieWeb', value: [
              $wizard.indieweb.webmention && 'Webmention',
              $wizard.indieweb.micropub && 'Micropub',
              $wizard.indieweb.indieauth && 'IndieAuth',
            ].filter(Boolean).join(', ') || 'None' },
          ]} />

          {#if createError}
            <p class="error">{createError}</p>
          {/if}

          <Choices>
            <Choice onclick={createSite} disabled={creating}>
              {creating ? 'Creating...' : 'Create site'}
            </Choice>
            <Choice muted onclick={startOver}>Start over</Choice>
          </Choices>
        </PalaceTurn>
      {/if}
    {/if}
  {/if}
</Thread>

<style>
  .greeting {
    font-size: 1.25rem;
    margin-bottom: 0.75rem;
    color: var(--color-text);
  }

  .site-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .site-item {
    font-family: var(--font-user);
    text-align: left;
    padding: 0.5rem 0.75rem;
    background: transparent;
    border: none;
    border-radius: var(--radius);
    color: var(--color-text);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
  }
  .site-item:hover {
    background: var(--color-surface);
  }

  /* Toggles — page-specific since only used here */
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

  .error { color: var(--color-danger); }

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
