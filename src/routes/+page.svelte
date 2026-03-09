<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { projects, loading, loadProjects } from '$lib/stores/projects';

  let ready = $state(false);

  onMount(async () => {
    await loadProjects();
    ready = true;
  });
</script>

<div class="conversation">
  <div class="message">
    <p class="greeting">Welcome to <strong>Fun Palace</strong>.</p>
    <p>Your sites, your web. What would you like to do?</p>
  </div>

  {#if ready}
    <div class="choices">
      <button class="choice" onclick={() => goto('/new/wizard')}>
        Create a new site
      </button>

      {#if $projects.length > 0}
        {#each $projects as project}
          <button class="choice" onclick={() => goto(`/project/${project.id}`)}>
            Work on {project.name}
          </button>
        {/each}
      {/if}

      <button class="choice choice-secondary" onclick={() => goto('/new/import')}>
        Import an existing site
      </button>
    </div>
  {/if}
</div>

<style>
  .conversation {
    max-width: 520px;
  }
  .message {
    font-family: var(--font-palace);
    margin-bottom: 2rem;
  }
  .greeting {
    font-size: 1.15rem;
    margin-bottom: 0.5rem;
  }
  .message p {
    color: var(--color-text-muted);
    line-height: 1.7;
    font-size: 0.875rem;
  }
  .message strong {
    color: var(--color-text);
    font-weight: 600;
  }
  .choices {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .choice {
    font-family: var(--font-user);
    text-align: left;
    padding: 0.75rem 1rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    color: var(--color-primary);
    font-weight: 500;
    font-size: 0.9rem;
    cursor: pointer;
    transition: border-color 0.15s;
  }
  .choice:hover {
    border-color: var(--color-primary);
  }
  .choice-secondary {
    color: var(--color-text-muted);
  }
</style>
