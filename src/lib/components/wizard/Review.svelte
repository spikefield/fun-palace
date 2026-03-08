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
