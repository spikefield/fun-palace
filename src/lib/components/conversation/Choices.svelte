<script>
  let { children } = $props();
</script>

<div class="choices">
  {@render children()}
</div>

<style>
  .choices {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.75rem;
  }
</style>
