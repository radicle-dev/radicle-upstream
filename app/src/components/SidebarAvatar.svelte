<script>
  import { Icon, Title } from "../DesignSystem";
  import SidebarTooltip from "./SidebarTooltip.svelte";
  import { link } from "svelte-spa-router";

  export let page = {};
</script>

<style>
  img {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }
</style>

<a href={page.path()} use:link>
  <img
    src="https://avatars2.githubusercontent.com/u/2326909?s=400&v=4"
    alt="user-avatar" />
</a>

<SidebarTooltip>
  <Title.Regular>{page.title}</Title.Regular>
</SidebarTooltip>
