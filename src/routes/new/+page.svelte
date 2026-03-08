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
