<script lang="ts">
  import { onMount } from 'svelte';
  import { projects, loading, error, loadProjects, removeProject } from '$lib/stores/projects';

  onMount(() => {
    loadProjects();
  });
</script>

<header>
  <h1>Twelvety</h1>
</header>

<div class="actions">
  <a href="/new/wizard" class="action-card">
    <div class="action-icon">&#10024;</div>
    <div>
      <h3>Create Your Site</h3>
      <p>Guided wizard — we'll explain everything along the way.</p>
    </div>
  </a>
  <a href="/new/import" class="action-card">
    <div class="action-icon">&#128194;</div>
    <div>
      <h3>Open Existing Site</h3>
      <p>Import an Eleventy project and enhance it.</p>
    </div>
  </a>
  <a href="/new/advanced" class="action-card">
    <div class="action-icon">&#9881;&#65039;</div>
    <div>
      <h3>Advanced Create</h3>
      <p>Quick form for experienced users.</p>
    </div>
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
          <p class="meta">{project.template_lang} &middot; {project.css_approach}</p>
          <p class="path">{project.path}</p>
        </div>
        <button class="btn-danger" onclick={() => removeProject(project.id)}>Remove</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  header {
    margin-bottom: 2rem;
  }
  .actions {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    margin-bottom: 2.5rem;
  }
  .action-card {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    background: var(--color-surface);
    border: 2px solid var(--color-border);
    border-radius: 12px;
    padding: 1.25rem;
    text-decoration: none;
    color: var(--color-text);
    transition: border-color 0.15s;
  }
  .action-card:hover { border-color: var(--color-primary); }
  .action-icon { font-size: 1.5rem; flex-shrink: 0; }
  .action-card h3 { font-size: 0.95rem; margin-bottom: 0.15rem; }
  .action-card p { color: var(--color-text-muted); font-size: 0.8rem; line-height: 1.4; }
  .section-title { font-size: 1.125rem; margin-bottom: 1rem; color: var(--color-text-muted); }
  .project-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .project-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 1rem 1.5rem;
    box-shadow: var(--shadow);
  }
  .project-card h3 { font-size: 1.125rem; }
  .project-card h3 a { color: var(--color-text); text-decoration: none; }
  .project-card h3 a:hover { color: var(--color-primary); }
  .meta { color: var(--color-text-muted); font-size: 0.875rem; }
  .path { color: var(--color-text-muted); font-size: 0.75rem; font-family: monospace; }
  .error { color: var(--color-danger); }
</style>
