<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
</script>

<div class="user-turn">
  {@render children()}
</div>

<style>
  .user-turn {
    font-family: var(--font-user);
    align-self: flex-end;
    background: var(--color-primary);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 999px;
    font-size: 0.8rem;
    font-weight: 500;
    max-width: 80%;
  }
</style>
