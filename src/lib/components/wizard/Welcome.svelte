<script lang="ts">
  import { goToStep } from '$lib/stores/wizard';
  import { goto } from '$app/navigation';

  function startWizard() {
    goToStep(1);
  }
</script>

<h1>Let's build your corner of the web</h1>
<p class="explanation">
  Fun Palace helps you create a personal website that <strong>you own</strong>.
  No algorithms, no ads, no platform lock-in — just your content, your way.
</p>

<div class="options">
  <button class="option" onclick={startWizard}>
    <div>
      <h3>Create a new site</h3>
      <p>I'll guide you through every step and explain what everything means.</p>
    </div>
  </button>

  <button class="option" onclick={() => goto('/new/import')}>
    <div>
      <h3>Open an existing Eleventy site</h3>
      <p>I'll detect your setup and offer to add indieweb features.</p>
    </div>
  </button>

  <button class="option" onclick={() => goto('/new/advanced')}>
    <div>
      <h3>Advanced create</h3>
      <p>I know what I'm doing — just give me the options.</p>
    </div>
  </button>
</div>

<style>
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; }
  .explanation strong { color: var(--color-text); }
  .options { display: flex; flex-direction: column; gap: 0.75rem; }
  .option {
    display: flex; align-items: center; gap: 1rem; text-align: left;
    background: var(--color-surface); border: 2px solid var(--color-border);
    border-radius: 12px; padding: 1.25rem; cursor: pointer;
    transition: border-color 0.15s; color: var(--color-text); width: 100%;
  }
  .option:hover { border-color: var(--color-primary); }
  .option h3 { font-size: 1rem; margin-bottom: 0.15rem; }
  .option p { color: var(--color-text-muted); font-size: 0.85rem; }
</style>
