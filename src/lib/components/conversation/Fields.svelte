<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
</script>

<div class="fields">
  {@render children()}
</div>

<style>
  .fields {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.75rem;
  }
  .fields :global(label) {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .fields :global(.field-label) {
    font-family: var(--font-palace);
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }
  .fields :global(.optional) {
    opacity: 0.6;
  }
  .fields :global(input) {
    font-family: var(--font-user);
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    font-size: 0.85rem;
    background: var(--color-surface);
    color: var(--color-text);
  }
  .fields :global(input:focus) {
    outline: none;
    border-color: var(--color-primary);
  }
  .fields :global(.dir-picker) {
    display: flex;
    gap: 0.5rem;
  }
  .fields :global(.dir-picker input) {
    flex: 1;
  }
</style>
