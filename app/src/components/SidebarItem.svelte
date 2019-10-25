<script>
  import { Icon, Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link, location } from "svelte-spa-router";

  export let href = "";
  export let title = "";
  export let icon = Icon.Search;
</script>

<a {href} use:link>
  <svelte:component this={icon} active={$location.startsWith(href)} />
</a>

<SidebarTooltip>
  <Title.Regular style="white-space: nowrap;">{title}</Title.Regular>
</SidebarTooltip>
