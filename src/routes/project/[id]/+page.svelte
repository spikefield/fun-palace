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
