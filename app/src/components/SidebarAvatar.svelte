<script>
  import { Icon, Text } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link } from "svelte-spa-router";

  export let image = null;
  export let title = null;
  export let href = null;
  export let active = null;

  export let dataCy = null;
</script>

<style>
  img {
    width: 34px;
    height: 34px;
    border-radius: 17px;
    border: 2px solid transparent;
  }

  .active {
    border: 2px solid var(--color-purple);
  }
</style>

<a {href} use:link data-cy={dataCy}>
  <img class:active src={image} alt="user-avatar" />
</a>

<SidebarTooltip>
  <Text variant="title">{title}</Text>
</SidebarTooltip>
