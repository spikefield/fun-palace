<script lang="ts">
  import { onMount } from 'svelte';
  import { wizard, resetWizard } from '$lib/stores/wizard';
  import WizardShell from '$lib/components/wizard/WizardShell.svelte';
  import Welcome from '$lib/components/wizard/Welcome.svelte';
  import Identity from '$lib/components/wizard/Identity.svelte';
  import SiteBasics from '$lib/components/wizard/SiteBasics.svelte';
  import TemplateLang from '$lib/components/wizard/TemplateLang.svelte';
  import Styling from '$lib/components/wizard/Styling.svelte';
  import IndieWeb from '$lib/components/wizard/IndieWeb.svelte';
  import Review from '$lib/components/wizard/Review.svelte';

  onMount(() => resetWizard());
</script>

<WizardShell>
  {#if $wizard.step === 0}
    <Welcome />
  {:else if $wizard.step === 1}
    <Identity />
  {:else if $wizard.step === 2}
    <SiteBasics />
  {:else if $wizard.step === 3}
    <TemplateLang />
  {:else if $wizard.step === 4}
    <Styling />
  {:else if $wizard.step === 5}
    <IndieWeb />
  {:else if $wizard.step === 6}
    <Review />
  {/if}
</WizardShell>
