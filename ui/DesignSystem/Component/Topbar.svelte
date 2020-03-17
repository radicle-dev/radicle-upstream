<script>
  import { link } from "svelte-spa-router";

  import { Avatar } from "../Primitive";
  import Menu from "./Topbar/Menu.svelte";

  export let style = null;

  export let avatarUrl = null;
  export let name = null;
  export let href = null;
  export let menuItems = null;
</script>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    background-color: var(--color-white);
    width: calc(100vw - var(--sidebar-width));
    height: var(--topbar-height);
    left: var(--sidebar-width);
    border-bottom: 1px solid var(--color-lightgray);
  }

  .name {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-lightgray);
    padding-left: 16px;
    padding-right: 24px;
  }

  .left {
    display: flex;
    flex: 1;
    justify-content: flex-end;
    margin-right: 16px;
  }
</style>

<div data-cy="topbar" class="topbar" {style}>
  <a class="name" {href} use:link>
    <Avatar
      style="color: var(--color-purple)"
      title={name}
      imageUrl={avatarUrl}
      variant="user" />
  </a>

  <Menu items={menuItems} />

  <div class="left">
    <slot />
  </div>
</div>
