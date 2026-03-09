<script lang="ts">
  import { wizard, totalSteps, nextStep, prevStep, canAdvance } from '$lib/stores/wizard';

  let { children } = $props();
</script>

<div class="wizard">
  <div class="progress">
    {#each Array(totalSteps) as _, i}
      <div class="dot" class:active={$wizard.step === i} class:done={i < $wizard.step}></div>
    {/each}
  </div>

  <div class="step-label">Step {$wizard.step + 1} of {totalSteps}</div>

  <div class="content">
    {@render children()}
  </div>

  <div class="nav">
    {#if $wizard.step > 0}
      <button class="btn btn-back" onclick={prevStep}>&larr; Back</button>
    {/if}
    {#if $wizard.step < totalSteps - 1}
      <button class="btn btn-next" onclick={nextStep} disabled={!$canAdvance}>Next &rarr;</button>
    {/if}
  </div>
</div>

<style>
  .wizard { max-width: 600px; margin: 0 auto; }
  .progress { display: flex; gap: 0.5rem; margin-bottom: 1.5rem; }
  .dot { width: 10px; height: 10px; border-radius: 50%; background: var(--color-border); }
  .dot.active { background: var(--color-primary); }
  .dot.done { background: #4ade80; }
  .step-label { font-size: 0.75rem; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 1rem; }
  .content { margin-bottom: 2rem; }
  .nav { display: flex; justify-content: space-between; }
  .btn { padding: 0.75rem 1.5rem; border-radius: var(--radius); font-weight: 600; }
  .btn-back { background: transparent; color: var(--color-text-muted); }
  .btn-next { background: var(--color-primary); color: white; }
  .btn-next:hover:not(:disabled) { background: var(--color-primary-hover); }
  .btn-next:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
