<script lang="ts">
  import { wizard, updateIndieweb } from '$lib/stores/wizard';
</script>

<div class="illustration">&#127760;</div>
<h1>Welcome to the IndieWeb</h1>
<p class="explanation">
  <strong>The IndieWeb is a movement to take back your content from big platforms.</strong>
  Instead of posting on Twitter and hoping it stays there, you publish on your own
  site first, then optionally share to social media. You're in control.
</p>

<div class="principles">
  <div class="principle">
    <strong>&#127968; Own your data</strong>
    <p>Your posts, photos, and thoughts live on YOUR domain. No platform can delete them or change the rules.</p>
  </div>
  <div class="principle">
    <strong>&#128257; POSSE</strong>
    <p>Publish on your Own Site, Syndicate Elsewhere. Post on your blog first, then share copies to social media.</p>
  </div>
  <div class="principle">
    <strong>&#128279; Connected but independent</strong>
    <p>IndieWeb sites can talk to each other — replies, likes, and mentions work across different websites.</p>
  </div>
</div>

<h2>Enable IndieWeb features</h2>
<p class="explanation">
  These add special tags to your site that enable cross-site communication.
  They're just placeholders for now — we'll set up the real endpoints later.
</p>

<div class="toggles">
  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.webmention}
      onchange={(e) => updateIndieweb({ webmention: e.currentTarget.checked })}>
    <div>
      <strong>Webmention</strong>
      <p>Like @mentions but across websites. When someone replies to your post from their blog, you'll know about it.</p>
    </div>
  </label>

  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.micropub}
      onchange={(e) => updateIndieweb({ micropub: e.currentTarget.checked })}>
    <div>
      <strong>Micropub</strong>
      <p>Post to your site from any app — not just Twelvety. It's like having an API for your blog.</p>
    </div>
  </label>

  <label class="toggle">
    <input type="checkbox" checked={$wizard.indieweb.indieauth}
      onchange={(e) => updateIndieweb({ indieauth: e.currentTarget.checked })}>
    <div>
      <strong>IndieAuth</strong>
      <p>Sign in to other websites using your own domain. Your URL is your username.</p>
    </div>
  </label>
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  h2 { font-size: 1.25rem; margin: 1.5rem 0 0.5rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 1rem; }
  .explanation strong { color: var(--color-text); }
  .principles { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 1.5rem; }
  .principle {
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem;
  }
  .principle strong { display: block; margin-bottom: 0.25rem; }
  .principle p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; }
  .toggles { display: flex; flex-direction: column; gap: 0.75rem; }
  .toggle {
    display: flex; gap: 0.75rem; align-items: flex-start;
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius); padding: 1rem; cursor: pointer;
  }
  .toggle input { margin-top: 0.25rem; flex-shrink: 0; width: 18px; height: 18px; }
  .toggle strong { display: block; margin-bottom: 0.15rem; }
  .toggle p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; }
</style>
