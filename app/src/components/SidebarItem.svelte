<script>
  import { Text } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link } from "svelte-spa-router";

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

<SidebarTooltip>
  <Text variant="title" style="white-space: nowrap;">{title}</Text>
</SidebarTooltip>
