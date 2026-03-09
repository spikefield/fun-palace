<script>
  let { done = false, children } = $props();
</script>

<div class="palace-turn" class:done>
  {@render children()}
</div>

<style>
  .palace-turn {
    font-family: var(--font-palace);
  }
  .palace-turn.done {
    opacity: 0.5;
  }
  .palace-turn :global(p) {
    color: var(--color-text-muted);
    line-height: 1.8;
    font-size: 0.85rem;
  }
  .palace-turn :global(strong) {
    color: var(--color-text);
    font-weight: 600;
  }
</style>
