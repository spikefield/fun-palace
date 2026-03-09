<script lang="ts">
  import { wizard } from '$lib/stores/wizard';

  const options = [
    {
      value: 'vanilla',
      name: 'Vanilla CSS',
      recommended: true,
      desc: 'Plain CSS — no build tools, no dependencies. Write styles that work everywhere. The simplest way to start.',
      tag: 'Zero config, universal',
    },
    {
      value: 'tailwind',
      name: 'Tailwind CSS',
      recommended: false,
      desc: 'Utility-first CSS framework. Style elements with class names like "text-lg" and "bg-blue-500". Very popular, fast iteration.',
      tag: 'Popular, utility-first',
    },
    {
      value: 'sass',
      name: 'Sass',
      recommended: false,
      desc: 'CSS with superpowers: variables, nesting, mixins, and functions. Compiles to regular CSS. Great for larger sites.',
      tag: 'Powerful, mature ecosystem',
    },
  ];

  function select(value: string) {
    wizard.update((s) => ({ ...s, css: value }));
  }
</script>

<div class="illustration">&#127912;</div>
<h1>How do you want to style your site?</h1>
<p class="explanation">
  <strong>CSS</strong> (Cascading Style Sheets) is how you control the look of your website —
  colors, fonts, layout, spacing. There are different tools that make writing CSS easier
  or more powerful. Don't worry — you can always change this later.
</p>

<div class="options">
  {#each options as opt}
    <button
      class="option"
      class:selected={$wizard.css === opt.value}
      onclick={() => select(opt.value)}
    >
      <h3>{opt.name} {#if opt.recommended}<span class="badge">Recommended</span>{/if}</h3>
      <p>{opt.desc}</p>
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
  .tag { font-size: 0.7rem; color: var(--color-text-muted); background: var(--color-border); padding: 0.15rem 0.5rem; border-radius: 4px; }
</style>
