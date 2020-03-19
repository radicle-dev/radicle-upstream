<script>
  import { location } from "svelte-spa-router";
  import { link } from "svelte-spa-router";
  import { Icon, Title } from "../Primitive";
  import IdentityAvatar from "./IdentityAvatar.svelte";

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
    background-color: var(--color-black);
    color: var(--color-white);
    text-align: center;
    border-radius: 2px;
    padding: 4px 8px 6px 8px;

    position: absolute;
    opacity: 0;
    top: 2px;
    left: 48px;
    pointer-events: none;
  }

  .tooltip:before {
    content: "";
    display: block;
    width: 0;
    height: 0;
    position: absolute;

    border-top: 6px solid transparent;
    border-bottom: 6px solid transparent;
    border-right: 6px solid var(--color-black);
    left: -6px;
    border-top-left-radius: 30%;

    top: 10px;
  }

  a {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
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
      class:active={path.active(path.profile(), $location, true)}>
      <a href={path.profileProjects()} use:link>
        <IdentityAvatar size="medium" showTitle={false} />
      </a>

      <div class="tooltip">
        <Title>Profile</Title>
      </div>
    </li>
  </ul>
</div>
