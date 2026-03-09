<script lang="ts">
  import { wizard, updateIdentity } from '$lib/stores/wizard';
</script>

<div class="illustration">&#128100;</div>
<h1>Who are you on the web?</h1>
<p class="explanation">
  On the indieweb, <strong>your identity lives on your own domain</strong> — not
  on Twitter, Facebook, or any other platform. Your site will include an
  <strong>h-card</strong>: a machine-readable digital business card that tells
  other websites who you are.
</p>
<p class="explanation">
  Think of it like this: when you comment on someone's blog or reply from your
  site, they'll see your name, photo, and a link back to you — all pulled from
  your h-card automatically.
</p>

<div class="fields">
  <label>
    Your Name <span class="required">*</span>
    <input type="text" value={$wizard.identity.name}
      oninput={(e) => updateIdentity({ name: e.currentTarget.value })}
      placeholder="Alice Example">
  </label>

  <label>
    Your Website URL <span class="required">*</span>
    <input type="url" value={$wizard.identity.url}
      oninput={(e) => updateIdentity({ url: e.currentTarget.value })}
      placeholder="https://alice.example.com">
    <span class="hint">This becomes your identity on the indieweb. Don't have one yet? Pick what you want it to be.</span>
  </label>

  <label>
    Email <span class="optional">(optional)</span>
    <input type="email" value={$wizard.identity.email}
      oninput={(e) => updateIdentity({ email: e.currentTarget.value })}
      placeholder="alice@example.com">
  </label>

  <label>
    Photo URL <span class="optional">(optional)</span>
    <input type="text" value={$wizard.identity.photo}
      oninput={(e) => updateIdentity({ photo: e.currentTarget.value })}
      placeholder="/img/avatar.jpg">
    <span class="hint">A URL to your profile photo. You can change this later.</span>
  </label>
</div>

{#if $wizard.identity.name}
  <div class="preview">
    <div class="preview-label">Preview: Your h-card</div>
    <div class="h-card-preview">
      <strong>{$wizard.identity.name}</strong>
      {#if $wizard.identity.url}
        <span class="url">{$wizard.identity.url}</span>
      {/if}
    </div>
  </div>
{/if}

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1rem; }
  .explanation strong { color: var(--color-text); }
  .fields { display: flex; flex-direction: column; gap: 1rem; margin-bottom: 1.5rem; }
  label { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.875rem; font-weight: 500; }
  input {
    padding: 0.5rem; border: 1px solid var(--color-border); border-radius: var(--radius);
    font-size: 0.875rem; background: var(--color-surface); color: var(--color-text);
  }
  .required { color: var(--color-danger); }
  .optional { color: var(--color-text-muted); font-weight: 400; }
  .hint { font-size: 0.75rem; color: var(--color-text-muted); font-weight: 400; }
  .preview { background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 1rem; }
  .preview-label { font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem; text-transform: uppercase; letter-spacing: 0.05em; }
  .h-card-preview strong { display: block; }
  .url { font-size: 0.85rem; color: var(--color-primary); }
</style>
