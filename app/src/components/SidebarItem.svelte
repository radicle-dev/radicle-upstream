<script>
  import { Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link } from "svelte-spa-router";

  export let icon = null;
  export let title = null;
  export let href = null;
  export let active = null;
</script>

<style>
  a.active :global(svg) {
    fill: var(--color-purple);
  }
</style>

<a {href} use:link class:active {title}>
  <svelte:component this={icon} />
</a>

<SidebarTooltip>
  <Title.Regular style="white-space: nowrap;">{title}</Title.Regular>
</SidebarTooltip>
