<script lang="ts">
  import { wizard, updateSite } from '$lib/stores/wizard';

  async function pickDirectory() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true });
    if (selected) {
      updateSite({ directory: selected as string });
    }
  }
</script>

<div class="illustration">&#128193;</div>
<h1>Name your site</h1>
<p class="explanation">
  Every site needs a name and a home on your computer. This creates a folder
  with all your site's files — <strong>you own them completely</strong>. No cloud
  service, no subscription. If you ever stop using Twelvety, your files are still
  there as a standard Eleventy project.
</p>

<div class="fields">
  <label>
    Site Name <span class="required">*</span>
    <input type="text" value={$wizard.site.name}
      oninput={(e) => updateSite({ name: e.currentTarget.value })}
      placeholder="Alice's Garden">
  </label>

  <label>
    Save Location <span class="required">*</span>
    <div class="dir-picker">
      <input type="text" value={$wizard.site.directory} placeholder="Choose a folder..." readonly>
      <button class="btn-primary" onclick={pickDirectory}>Browse</button>
    </div>
    <span class="hint">Your site will be created in a subfolder here.</span>
  </label>
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1.5rem; }
  .explanation strong { color: var(--color-text); }
  .fields { display: flex; flex-direction: column; gap: 1rem; }
  label { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.875rem; font-weight: 500; }
  input {
    padding: 0.5rem; border: 1px solid var(--color-border); border-radius: var(--radius);
    font-size: 0.875rem; background: var(--color-surface); color: var(--color-text);
  }
  .dir-picker { display: flex; gap: 0.5rem; }
  .dir-picker input { flex: 1; }
  .required { color: var(--color-danger); }
  .hint { font-size: 0.75rem; color: var(--color-text-muted); font-weight: 400; }
</style>
