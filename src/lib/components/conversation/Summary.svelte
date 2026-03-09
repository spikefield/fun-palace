<script>
  let { rows }: { rows: Array<{ label: string; value: string; mono?: boolean }> } = $props();
</script>

<div class="summary">
  {#each rows as row}
    <div class="row">
      <span class="label">{row.label}</span>
      <span class="value" class:mono={row.mono}>{row.value}</span>
    </div>
  {/each}
</div>

<style>
  .summary {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 0.75rem;
    margin-top: 0.75rem;
    font-family: var(--font-user);
  }
  .row {
    display: flex;
    justify-content: space-between;
    padding: 0.35rem 0;
    border-bottom: 1px solid var(--color-border);
  }
  .row:last-child { border-bottom: none; }
  .label { color: var(--color-text-muted); font-size: 0.8rem; }
  .value { font-weight: 500; font-size: 0.8rem; }
  .mono { font-family: var(--font-palace); font-size: 0.75rem; }
</style>
