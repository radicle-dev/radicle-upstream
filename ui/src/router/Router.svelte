<script lang="typescript">
  import { location } from "ui/src/router";

  export let routes;
  export let initial;

  $: component = routes[$location || initial];
</script>

<svelte:component this={component} />
