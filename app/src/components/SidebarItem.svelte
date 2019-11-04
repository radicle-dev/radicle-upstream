<script>
  import { Icon, Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link, location } from "svelte-spa-router";

  export let href = "";
  export let title = "";
  export let icon = Icon.Search;
</script>

<style>
  :global(.active svg) {
    fill: var(--color-purple);
  }
</style>

<a {href} use:link class={$location.startsWith(href) ? 'active' : ''}>
  <svelte:component this={icon} />
</a>

<SidebarTooltip>
  <Title.Regular style="white-space: nowrap;">{title}</Title.Regular>
</SidebarTooltip>
