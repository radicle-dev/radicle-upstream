<script lang="ts">
  import * as navigation from "./src/navigation";

  export let nav: navigation.Navigation<string>;
  const view = nav.current;
</script>

{#if $view.props}
  <svelte:component this={$view.component} {...$view.props} />
{:else}
  <svelte:component this={$view.component} />
{/if}
