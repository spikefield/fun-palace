<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { Thread, PalaceTurn, Choices, Choice, Summary } from '$lib/components/conversation';
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
    } catch (e: unknown) {
      const err = e as { message?: string };
      errorMsg = err.message ?? 'Failed to load project';
      return;
    }
    if (project) {
      try {
        serving = await invoke<boolean>('eleventy_status', { projectId: data.projectId });
      } catch {
        // Status check can fail silently — just means not serving
        serving = false;
      }
    }
  }

  async function startServe() {
    try {
      await invoke('eleventy_serve', { projectId: data.projectId });
      serving = true;
      setTimeout(async () => {
        const { open } = await import('@tauri-apps/plugin-shell');
        await open('http://localhost:8080');
      }, 2000);
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

<Thread>
  {#if !project}
    <PalaceTurn><p>Loading project...</p></PalaceTurn>
  {:else}
    <PalaceTurn>
      <p class="project-name">{project.name}</p>
      <Summary rows={[
        { label: 'Location', value: String(project.path), mono: true },
        { label: 'Templates', value: project.template_lang },
        { label: 'Styling', value: project.css_approach },
      ]} />
    </PalaceTurn>

    <PalaceTurn>
      <p>What would you like to do?</p>
      <Choices>
        {#if serving}
          <Choice onclick={stopServe}>Stop dev server</Choice>
        {:else}
          <Choice onclick={startServe}>Start dev server</Choice>
        {/if}
        <Choice onclick={runBuild} disabled={building}>
          {building ? 'Building...' : 'Build'}
        </Choice>
        <Choice muted onclick={() => goto('/')}>Back to home</Choice>
      </Choices>
    </PalaceTurn>

    {#if errorMsg}
      <PalaceTurn>
        <p>Something went wrong:</p>
        <pre class="output error">{errorMsg}</pre>
        <Choices>
          <Choice onclick={() => { errorMsg = ''; }}>Dismiss</Choice>
        </Choices>
      </PalaceTurn>
    {/if}

    {#if buildOutput}
      <PalaceTurn>
        <p>Build complete:</p>
        <pre class="output">{buildOutput}</pre>
      </PalaceTurn>
    {/if}
  {/if}
</Thread>

<style>
  .project-name {
    font-size: 1.25rem;
    color: var(--color-text);
    font-weight: 600;
  }
  .error { color: var(--color-danger); }
  .output {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 1rem;
    font-size: 0.75rem;
    font-family: var(--font-palace);
    overflow-x: auto;
    white-space: pre-wrap;
    margin-top: 0.5rem;
  }
</style>
