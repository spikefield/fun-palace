<script>
  import '../app.css';
  let { children } = $props();
</script>

<div class="app">
  <div class="content">
    {@render children()}
  </div>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  .content {
    flex: 1;
    max-width: 600px;
    width: 100%;
    margin: 0 auto;
    padding: 0 1.5rem 3rem;
  }
</style>
