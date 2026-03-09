<script lang="ts">
  import { onMount } from 'svelte';
  import { projects, loading, error, loadProjects, removeProject } from '$lib/stores/projects';

  onMount(() => {
    loadProjects();
  });
</script>

<header>
  <h1>Fun Palace</h1>
  <p class="tagline">Your sites, your web.</p>
</header>

<div class="actions">
  <a href="/new/wizard" class="action-card">
    <h3>Create</h3>
    <p>Guided setup — we'll walk you through it.</p>
  </a>
  <a href="/new/import" class="action-card">
    <h3>Import</h3>
    <p>Bring in an existing Eleventy project.</p>
  </a>
  <a href="/new/advanced" class="action-card">
    <h3>Quick Start</h3>
    <p>All options on one form.</p>
  </a>
</div>

{#if $loading}
  <p>Loading projects...</p>
{:else if $error}
  <p class="error">{$error}</p>
{:else if $projects.length > 0}
  <h2 class="section-title">Your Sites</h2>
  <div class="project-list">
    {#each $projects as project}
      <div class="project-card">
        <div>
          <h3><a href="/project/{project.id}">{project.name}</a></h3>
          <p class="meta">{project.template_lang} · {project.css_approach}</p>
          <p class="path">{project.path}</p>
        </div>
        <button class="btn-danger" onclick={() => removeProject(project.id)}>Remove</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  header {
    margin-bottom: 2.5rem;
  }
  h1 {
    font-size: 1.75rem;
    font-weight: 700;
  }
  .tagline {
    color: var(--color-text-muted);
    font-size: 0.9rem;
    margin-top: 0.25rem;
  }
  .actions {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1px;
    background: var(--color-border);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    overflow: hidden;
    margin-bottom: 2.5rem;
  }
  .action-card {
    background: var(--color-surface);
    padding: 1.25rem;
    text-decoration: none;
    color: var(--color-text);
    transition: background 0.15s;
  }
  .action-card:hover { background: var(--color-bg); }
  .action-card h3 {
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--color-primary);
    margin-bottom: 0.35rem;
  }
  .action-card p { color: var(--color-text-muted); font-size: 0.8rem; line-height: 1.4; }
  .section-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
    margin-bottom: 0.75rem;
  }
  .project-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
    background: var(--color-border);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    overflow: hidden;
  }
  .project-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--color-surface);
    padding: 1rem 1.25rem;
  }
  .project-card h3 { font-size: 1rem; font-weight: 600; }
  .project-card h3 a { color: var(--color-text); text-decoration: none; }
  .project-card h3 a:hover { color: var(--color-primary); }
  .meta { color: var(--color-text-muted); font-size: 0.8rem; }
  .path { color: var(--color-text-muted); font-size: 0.7rem; font-family: monospace; }
  .error { color: var(--color-danger); }
</style>
