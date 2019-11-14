<script>
  import { Icon, Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link } from "svelte-spa-router";

  export let href = "";
  export let title = "";
  export let image = "";
</script>

<style>
  img {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }
</style>

<a {href} use:link>
  <img src={image} alt="user-avatar" />
</a>

<SidebarTooltip>
  <Title.Regular>{title}</Title.Regular>
</SidebarTooltip>
