<script lang="ts">
  import * as view from "../src/view";

  export let nav: view.Navigation<string>;
  const current = nav.current;
</script>

{#if $current.props}
  <svelte:component this={$current.component} {...$current.props} />
{:else}
  <svelte:component this={$current.component} />
{/if}
