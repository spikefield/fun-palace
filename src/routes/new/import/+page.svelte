<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';

  interface DetectionResult {
    is_eleventy: boolean;
    config_file: string | null;
    template_lang: string | null;
    post_count: number;
    has_indieweb_markup: boolean;
    has_funpalace_config: boolean;
    has_feeds: boolean;
  }

  type Stage = 'pick' | 'detecting' | 'report' | 'error';

  let stage = $state<Stage>('pick');
  let directory = $state('');
  let result = $state<DetectionResult | null>(null);
  let errorMsg = $state('');
  let registering = $state(false);

  async function pickDirectory() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      directory = selected as string;
      await detect();
    }
  }

  async function detect() {
    stage = 'detecting';
    errorMsg = '';
    try {
      result = await invoke<DetectionResult>('detect_eleventy_project', { directory });
      stage = 'report';
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Detection failed';
      stage = 'error';
    }
  }

  async function registerProject() {
    if (!result || !directory) return;
    registering = true;
    try {
      const name = directory.split('/').pop() || 'Imported Site';
      await invoke('add_project', {
        name,
        path: directory,
        templateLang: result.template_lang || 'nunjucks',
        cssApproach: 'vanilla',
      });
      goto('/');
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to register project';
    } finally {
      registering = false;
    }
  }
</script>

<header>
  <a href="/">&larr; Dashboard</a>
  <h1>Open Existing Site</h1>
</header>

{#if stage === 'pick'}
  <div class="center">
    <h2>Choose your Eleventy project folder</h2>
    <p class="explanation">
      Select the root directory of your existing Eleventy site.
      We'll scan it to detect your setup and offer to enhance it with indieweb features.
    </p>
    <button class="btn-primary btn-large" onclick={pickDirectory}>Browse for folder</button>
  </div>

{:else if stage === 'detecting'}
  <div class="center">
    <div class="spinner"></div>
    <p>Scanning {directory}...</p>
  </div>

{:else if stage === 'report' && result}
  <div class="report">
    {#if !result.is_eleventy}
      <div class="warning">
        <strong>This doesn't look like an Eleventy project</strong>
        <p>No eleventy config file or @11ty/eleventy dependency was found. You can still register it, but it may not work as expected.</p>
      </div>
    {/if}

    <h2>Detection Results</h2>
    <div class="results">
      <div class="row">
        <span class="label">Eleventy project</span>
        <span class="value">{result.is_eleventy ? 'Yes' : 'No'}</span>
      </div>
      {#if result.config_file}
        <div class="row">
          <span class="label">Config file</span>
          <span class="value mono">{result.config_file}</span>
        </div>
      {/if}
      <div class="row">
        <span class="label">Template language</span>
        <span class="value">{result.template_lang || 'Not detected'}</span>
      </div>
      <div class="row">
        <span class="label">Content files</span>
        <span class="value">{result.post_count} markdown files</span>
      </div>
      <div class="row">
        <span class="label">IndieWeb markup</span>
        <span class="value">{result.has_indieweb_markup ? 'Found' : 'Not found'}</span>
      </div>
      <div class="row">
        <span class="label">Fun Palace config</span>
        <span class="value">{result.has_funpalace_config ? 'Found' : 'Not found'}</span>
      </div>
      <div class="row">
        <span class="label">Feeds</span>
        <span class="value">{result.has_feeds ? 'Found' : 'Not found'}</span>
      </div>
    </div>

    <div class="actions">
      <button class="btn-primary btn-large" onclick={registerProject} disabled={registering}>
        {registering ? 'Adding...' : 'Add to Fun Palace'}
      </button>
      <button class="btn-secondary" onclick={() => { stage = 'pick'; result = null; }}>
        Choose different folder
      </button>
    </div>
  </div>

{:else if stage === 'error'}
  <div class="center">
    <p class="error">{errorMsg}</p>
    <button class="btn-secondary" onclick={() => { stage = 'pick'; }}>Try again</button>
  </div>
{/if}

<style>
  header { margin-bottom: 2rem; }
  header a { color: var(--color-text-muted); text-decoration: none; }
  .center { text-align: center; padding: 3rem 0; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; max-width: 480px; margin-left: auto; margin-right: auto; }
  .btn-large { padding: 0.75rem 2rem; font-size: 1rem; }
  .btn-secondary { background: var(--color-surface); border: 1px solid var(--color-border); color: var(--color-text); padding: 0.5rem 1rem; border-radius: var(--radius); cursor: pointer; }
  .spinner {
    width: 32px; height: 32px; border: 3px solid var(--color-border);
    border-top-color: var(--color-primary); border-radius: 50%;
    animation: spin 0.8s linear infinite; margin: 0 auto 1rem;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
  .report { max-width: 520px; }
  .warning {
    background: #451a03; border: 1px solid #92400e; border-radius: var(--radius);
    padding: 1rem; margin-bottom: 1.5rem;
  }
  .warning p { color: var(--color-text-muted); font-size: 0.85rem; margin-top: 0.25rem; }
  .results {
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem; margin-bottom: 1.5rem;
  }
  .row { display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--color-border); }
  .row:last-child { border-bottom: none; }
  .label { color: var(--color-text-muted); font-size: 0.875rem; }
  .value { font-weight: 500; font-size: 0.875rem; }
  .mono { font-family: monospace; font-size: 0.8rem; }
  .actions { display: flex; flex-direction: column; gap: 0.75rem; }
  .error { color: var(--color-danger); margin-bottom: 1rem; }
</style>
