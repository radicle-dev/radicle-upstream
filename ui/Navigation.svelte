<script>
  import { store } from "./src/navigation.ts";

  export let views = null;

  let view;

  store.subscribe(val => {
    view = views[val];
  });

  $: console.log(view);
</script>

{#if view.props}
  <svelte:component this={view.component} {...view.props} />
{:else}
  <svelte:component this={view.component} />
{/if}
