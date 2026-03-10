<script lang="ts">
  import { wizard } from '$lib/stores/wizard';

  const options = [
    {
      value: 'webc',
      name: 'WebC',
      recommended: true,
      desc: 'Web-native components using plain HTML custom elements. If you know HTML, you know WebC.',
      preview: '<site-header></site-header>',
      tag: 'Eleventy-native, just HTML',
    },
    {
      value: 'nunjucks',
      name: 'Nunjucks',
      desc: 'The most popular choice for Eleventy. Template inheritance, macros, lots of examples.',
      preview: '{{ title }}',
      tag: 'Most popular',
    },
    {
      value: 'liquid',
      name: 'Liquid',
      desc: 'Simple and beginner-friendly. Originally from Shopify, also used by Jekyll.',
      preview: '{{ page.title }}',
      tag: 'Shopify, Jekyll',
    },
    {
      value: 'jsx',
      name: 'JSX',
      desc: 'React-style templates. Write components with JavaScript and JSX syntax.',
      preview: '<h1>{data.title}</h1>',
      tag: 'React-style',
    },
    {
      value: 'mdx',
      name: 'MDX',
      desc: 'Markdown with embedded JSX components. Mix prose with interactive elements.',
      preview: '# Hello <Counter />',
      tag: 'Markdown + JSX',
    },
    {
      value: 'typescript',
      name: 'TypeScript',
      desc: 'Type-safe JavaScript templates. Full TypeScript support in your template files.',
      preview: 'export default (data: Data) =>',
      tag: 'Type-safe',
    },
    {
      value: 'handlebars',
      name: 'Handlebars',
      desc: 'Logic-less templates with helpers. Familiar to many web developers.',
      preview: '{{#each posts}}',
      tag: 'Logic-less',
    },
    {
      value: 'pug',
      name: 'Pug',
      desc: 'Indentation-based HTML shorthand. Minimal syntax, no closing tags.',
      preview: 'h1= title',
      tag: 'Concise',
    },
    {
      value: 'mustache',
      name: 'Mustache',
      desc: 'Logic-less templates. The simplest template syntax — just variables and sections.',
      preview: '{{title}}',
      tag: 'Minimal',
    },
    {
      value: 'ejs',
      name: 'EJS',
      desc: 'Embedded JavaScript. Plain JS inside HTML with <% %> tags.',
      preview: '<%= title %>',
      tag: 'Express.js default',
    },
    {
      value: 'haml',
      name: 'HAML',
      desc: 'Clean, indentation-based markup. Popular in the Ruby world.',
      preview: '%h1= title',
      tag: 'Ruby-style',
    },
  ];

  function select(value: string) {
    wizard.update((s) => ({ ...s, templateLang: value }));
  }
</script>

<h1>How should your templates work?</h1>
<p class="explanation">
  Templates control how your content turns into web pages. Think of them like
  <strong>mail merge</strong> — you write your blog post in plain text, and the
  template wraps it in your site's design. You can always change later.
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
  h1 { font-size: 1.75rem; margin-bottom: 0.75rem; }
  .explanation { color: var(--color-text-muted); line-height: 1.7; margin-bottom: 2rem; }
  .explanation strong { color: var(--color-text); }
  .options { display: flex; flex-direction: column; gap: 0.5rem; }
  .option {
    text-align: left; width: 100%; background: var(--color-surface);
    border: 2px solid var(--color-border); border-radius: var(--radius);
    padding: 1rem; cursor: pointer; transition: all 0.15s; color: var(--color-text);
  }
  .option:hover { border-color: var(--color-primary); }
  .option.selected { border-color: var(--color-primary); background: var(--color-selected); }
  .option h3 { font-size: 0.9rem; margin-bottom: 0.2rem; display: flex; align-items: center; gap: 0.5rem; }
  .badge { font-size: 0.6rem; background: var(--color-primary); color: white; padding: 0.1rem 0.4rem; border-radius: 99px; font-weight: 600; }
  .option p { color: var(--color-text-muted); font-size: 0.8rem; line-height: 1.4; margin-bottom: 0.35rem; }
  .preview { display: inline-block; font-size: 0.75rem; background: var(--color-border); padding: 0.15rem 0.4rem; border-radius: 4px; margin-bottom: 0.35rem; }
  .tag { font-size: 0.65rem; color: var(--color-text-muted); background: var(--color-border); padding: 0.1rem 0.4rem; border-radius: 4px; }
</style>
