<script>
  import { link } from "svelte-spa-router";

  import { Text } from "../../Primitives";

  import Tooltip from "./Tooltip.svelte";

  export let icon = null;
  export let title = null;
  export let href = null;
  export let active = null;

  export let dataCy = null;
</script>

<style>
  a.active :global(svg) {
    fill: var(--color-purple);
  }
</style>

<a {href} use:link class:active data-cy={dataCy}>
  <svelte:component this={icon} />
</a>

<Tooltip>
  <Text variant="title" style="white-space: nowrap;">{title}</Text>
</Tooltip>
