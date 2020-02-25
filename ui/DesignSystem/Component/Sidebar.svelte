<script>
  import { location } from "svelte-spa-router";
  import { Icon } from "../Primitive";

  import Item from "./Sidebar/Item.svelte";
  import Avatar from "./Sidebar/Avatar.svelte";

  import * as path from "../../lib/path.js";
</script>

<style>
  .wrapper {
    width: var(--sidebar-width);
    height: 100%;
    background-color: #eeeeef;
    position: fixed;
    padding-top: 31px;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-right: 1px solid #e3e3e3;
  }

  li {
    width: 36px;
    height: 36px;
    margin-bottom: 12px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .menu-item:hover:before {
    position: absolute;
    content: " ";
    width: 4px;
    height: 44px;
    background-color: var(--color-gray);
    top: -5px;
    left: -16px;
    border-top-right-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  .menu-item.active:before {
    position: absolute;
    content: " ";
    width: 4px;
    height: 44px;
    background-color: var(--color-purple);
    top: -5px;
    left: -16px;
    border-top-right-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  .wrapper :global(li .show-on-hover) {
    opacity: 0;
    display: none;
  }

  .wrapper :global(li:hover svg) {
    fill: var(--color-purple);
  }

  .wrapper :global(li:hover .show-on-hover) {
    opacity: 1;
    display: inline-block;
  }

  .divider {
    width: 36px;
    height: 17px;
  }

  .line {
    height: 1px;
    width: 100%;
    background-color: var(--color-gray);
  }
</style>

<div class="wrapper" data-cy="sidebar">
  <ul class="top-menu">
    <li class="menu-item" class:active={path.active(path.search(), $location)}>
      <Item
        icon={Icon.Search}
        title="Search"
        dataCy="search"
        href={path.search()}
        active={path.active(path.search(), $location)} />
    </li>
    <li class="menu-item" class:active={path.active(path.network(), $location)}>
      <Item
        icon={Icon.Peer}
        title="Network"
        dataCy="network"
        href={path.network()}
        active={path.active(path.network(), $location)} />
    </li>
    <li class="divider">
      <div class="line" />
    </li>
    <li
      class="menu-item"
      class:active={path.active(path.projects(), $location)}>
      <Avatar
        image="https://avatars2.githubusercontent.com/u/2326909?s=400&v=4"
        title="Profile"
        dataCy="profile"
        href={path.projects()} />
    </li>
  </ul>
</div>
