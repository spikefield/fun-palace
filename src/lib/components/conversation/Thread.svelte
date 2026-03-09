<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
</script>

<div class="thread">
  {@render children()}
</div>

<style>
  .thread {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding-top: 15vh;
    padding-bottom: 4rem;
  }
</style>
