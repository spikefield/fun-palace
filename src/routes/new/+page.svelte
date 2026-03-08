<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => goto('/new/wizard'));
</script>

<p>Redirecting...</p>
