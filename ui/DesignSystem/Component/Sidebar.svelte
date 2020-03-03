<script>
  import { location } from "svelte-spa-router";
  import { link } from "svelte-spa-router";
  import { Icon, Title } from "../Primitive";
  import Avatar from "../Component/Avatar.svelte";

  import * as path from "../../lib/path.js";
</script>

<style>
  .wrapper {
    width: var(--sidebar-width);
    height: 100%;
    background-color: #eeeeef;
    position: fixed;
    padding-top: 31px;
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-right: 1px solid #e3e3e3;
  }

  .item {
    width: 36px;
    height: 36px;
    margin-bottom: 12px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .item:hover:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 44px;
    background-color: var(--color-gray);
    top: -4px;
    left: -16px;
    border-top-right-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  .item.active:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 44px;
    background-color: var(--color-purple);
    top: -4px;
    left: -16px;
    border-top-right-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  .wrapper :global(li:hover svg) {
    fill: var(--color-purple);
  }

  .item.active :global(svg) {
    fill: var(--color-purple);
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

  .item:hover .tooltip {
    opacity: 1;
    transition: 0.3s;
  }

  .tooltip {
    user-select: none;
    background-color: var(--color-white);
    border: 1px solid var(--color-lightgray);
    color: var(--color-darkgray);
    text-align: center;
    border-radius: 2px;
    padding: 4px 10px 6px 8px;
    box-shadow: 0px 4px 8px var(--color-lightgray-opacity-08);

    position: absolute;
    opacity: 0;
    top: 2px;
    left: 48px;
    pointer-events: none;
  }

  .tooltip:after,
  .tooltip:before {
    right: 100%;
    top: 50%;
    border: solid transparent;
    content: "";
    height: 0;
    width: 0;
    position: absolute;
  }

  .tooltip:after {
    border-right-color: var(--color-white);
    border-width: 6px;
    margin-top: -6px;
  }

  .tooltip:before {
    border-right-color: var(--color-lightgray);
    border-width: 7px;
    margin-top: -7px;
  }

  a {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .user-avatar {
    width: 36px;
    height: 36px;
    border-radius: 18px;
  }
</style>

<div class="wrapper" data-cy="sidebar">
  <ul>
    <li
      class="item"
      data-cy="search"
      class:active={path.active(path.search(), $location)}>
      <a href={path.search()} use:link>
        <Icon.Search />
      </a>

      <div class="tooltip">
        <Title style="white-space: nowrap;">Search</Title>
      </div>
    </li>

    <li
      class="item"
      data-cy="network"
      class:active={path.active(path.network(), $location)}>
      <a href={path.network()} use:link>
        <Icon.Peer />
      </a>

      <div class="tooltip">
        <Title style="white-space: nowrap;">Network</Title>
      </div>
    </li>

    <li class="divider">
      <div class="line" />
    </li>

    <li
      class="item"
      data-cy="profile"
      class:active={path.active(path.projects(), $location, true)}>
      <a href={path.projects()} use:link>
        <Avatar size="medium" variant="user" />
      </a>

      <div class="tooltip">
        <Title>Profile</Title>
      </div>
    </li>
  </ul>
</div>
