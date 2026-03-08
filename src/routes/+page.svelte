<script lang="ts">
  import { onMount } from 'svelte';
  import { projects, loading, error, loadProjects, removeProject } from '$lib/stores/projects';

  onMount(() => {
    loadProjects();
  });
</script>

<header>
  <h1>Twelvety</h1>
  <a href="/new"><button class="btn-primary">New Project</button></a>
</header>

{#if $loading}
  <p>Loading projects...</p>
{:else if $error}
  <p class="error">{$error}</p>
{:else if $projects.length === 0}
  <div class="empty">
    <p>No projects yet.</p>
    <a href="/new"><button class="btn-primary">Create your first site</button></a>
  </div>
{:else}
  <div class="project-list">
    {#each $projects as project}
      <div class="project-card">
        <div>
          <h2><a href="/project/{project.id}">{project.name}</a></h2>
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
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  .empty {
    text-align: center;
    padding: 4rem 0;
  }
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
  .project-card h2 { font-size: 1.125rem; }
  .project-card h2 a { color: var(--color-text); text-decoration: none; }
  .project-card h2 a:hover { color: var(--color-primary); }
  .meta { color: var(--color-text-muted); font-size: 0.875rem; }
  .path { color: var(--color-text-muted); font-size: 0.75rem; font-family: monospace; }
  .error { color: var(--color-danger); }
</style>
