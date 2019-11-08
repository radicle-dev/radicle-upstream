<script>
  import { Icon, Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link, location } from "svelte-spa-router";

  export let page = {};
</script>

<style>
  a.active :global(svg) {
    fill: var(--color-purple);
  }
</style>

<a
  href={page.path()}
  use:link
  class={$location.startsWith(page.path()) ? 'active' : ''}>
  <svelte:component this={page.icon} />
</a>

<SidebarTooltip>
  <Title.Regular style="white-space: nowrap;">{page.title}</Title.Regular>
</SidebarTooltip>
