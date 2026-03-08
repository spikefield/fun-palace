<script lang="ts">
  import { wizard } from '$lib/stores/wizard';

  const options = [
    {
      value: 'webc',
      name: 'WebC',
      recommended: true,
      desc: 'Web-native components using plain HTML custom elements. No special syntax to learn — if you know HTML, you know WebC. The simplest and most modern option.',
      preview: '<site-header></site-header>',
      tag: 'Eleventy-native, just HTML',
    },
    {
      value: 'nunjucks',
      name: 'Nunjucks',
      recommended: false,
      desc: 'The most popular choice for Eleventy. Powerful features like template inheritance and macros. Lots of documentation and community examples.',
      preview: '{{ title }}',
      tag: 'Most popular, well-documented',
    },
    {
      value: 'liquid',
      name: 'Liquid',
      recommended: false,
      desc: 'Simpler and beginner-friendly. Originally from Shopify. Great if you\'ve used Jekyll or Shopify themes before.',
      preview: '{{ page.title }}',
      tag: 'Used by: Shopify, Jekyll sites',
    },
  ];

  function select(value: string) {
    wizard.update((s) => ({ ...s, templateLang: value }));
  }
</script>

<div class="illustration">&#127912;</div>
<h1>How should your templates work?</h1>
<p class="explanation">
  Templates control how your content turns into web pages. Think of them like
  <strong>mail merge</strong> — you write your blog post in plain text, and the
  template wraps it in your site's design with headers, navigation, and footer.
  <br><br>
  Different template languages have different syntax for this. Don't worry —
  you can always change later.
</p>

<div class="options">
  {#each options as opt}
    <button
      class="option"
      class:selected={$wizard.templateLang === opt.value}
      onclick={() => select(opt.value)}
    >
      <h3>{opt.name} {#if opt.recommended}<span class="badge">Recommended</span>{/if}</h3>
      <p>{opt.desc}</p>
      <code class="preview">{opt.preview}</code>
      <span class="tag">{opt.tag}</span>
    </button>
  {/each}
</div>

<style>
  .illustration { font-size: 3rem; margin-bottom: 1rem; }
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; }
  .explanation strong { color: var(--color-text); }
  .options { display: flex; flex-direction: column; gap: 0.75rem; }
  .option {
    text-align: left; width: 100%; background: var(--color-surface);
    border: 2px solid var(--color-border); border-radius: 12px;
    padding: 1.25rem; cursor: pointer; transition: all 0.15s; color: var(--color-text);
  }
  .option:hover { border-color: var(--color-primary); }
  .option.selected { border-color: var(--color-primary); background: var(--color-selected); }
  .option h3 { font-size: 1rem; margin-bottom: 0.25rem; display: flex; align-items: center; gap: 0.5rem; }
  .badge { font-size: 0.65rem; background: #4ade80; color: #000; padding: 0.15rem 0.5rem; border-radius: 99px; font-weight: 600; }
  .option p { color: var(--color-text-muted); font-size: 0.85rem; line-height: 1.5; margin-bottom: 0.5rem; }
  .preview { display: inline-block; font-size: 0.8rem; background: var(--color-border); padding: 0.25rem 0.5rem; border-radius: 4px; margin-bottom: 0.5rem; }
  .tag { font-size: 0.7rem; color: var(--color-text-muted); background: var(--color-border); padding: 0.15rem 0.5rem; border-radius: 4px; }
</style>
