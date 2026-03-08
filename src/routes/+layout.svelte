<script>
  import '../app.css';
  let { children } = $props();
</script>

<div class="app">
  {@render children()}
</div>

<style>
  .app {
    max-width: 960px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
</style>
