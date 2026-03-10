<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    onclick,
    disabled = false,
    selected = false,
    muted = false,
    children,
  }: {
    onclick?: () => void;
    disabled?: boolean;
    selected?: boolean;
    muted?: boolean;
    children: Snippet;
  } = $props();
</script>

<button
  class="choice"
  class:selected
  class:muted
  {onclick}
  {disabled}
>
  {@render children()}
</button>

<style>
  .choice {
    font-family: var(--font-user);
    padding: 0.5rem 1rem;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: 999px;
    color: var(--color-text);
    font-weight: 500;
    font-size: 0.8rem;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
  }
  .choice:hover:not(:disabled) {
    border-color: var(--color-primary);
    background: var(--color-surface);
  }
  .choice:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .choice.selected {
    border-color: var(--color-primary);
    background: var(--color-primary);
    color: white;
  }
  .choice.muted {
    color: var(--color-text-muted);
  }
</style>
