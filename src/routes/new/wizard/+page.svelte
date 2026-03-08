<script lang="ts">
  import { onMount } from 'svelte';
  import { wizard, resetWizard } from '$lib/stores/wizard';
  import WizardShell from '$lib/components/wizard/WizardShell.svelte';
  import Welcome from '$lib/components/wizard/Welcome.svelte';

  onMount(() => resetWizard());
</script>

<WizardShell>
  {#if $wizard.step === 0}
    <Welcome />
  {:else}
    <p>Step {$wizard.step + 1} coming soon...</p>
  {/if}
</WizardShell>
