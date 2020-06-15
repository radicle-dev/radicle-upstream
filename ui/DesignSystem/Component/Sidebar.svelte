<script>
  import { createEventDispatcher } from "svelte";
  import { location, link } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Avatar, Icon, Title } from "../Primitive";

  import AddOrgButton from "./Sidebar/AddOrgButton.svelte";

  const dispatch = createEventDispatcher();

  export let identity = null;
  export let orgs = null;
</script>

<style>
  .wrapper {
    width: var(--sidebar-width);
    height: 100%;
    background-color: var(--color-foreground-level-2);
    position: fixed;
    padding-top: 16px;
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
  }
  .top {
    overflow-y: scroll;
    padding-bottom: 36px;
  }
  .top::-webkit-scrollbar {
    display: none;
  }

  .bottom {
    position: relative;
    padding-top: 16px;
  }

  .bottom:before {
    position: absolute;
    content: " ";
    height: 36px;
    width: 36px;
    top: -36px;
    left: 0;
    background: linear-gradient(
      0deg,
      rgba(235, 239, 243, 1) 0%,
      rgba(235, 239, 243, 0) 100%
    );
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

  .indicator:hover:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 44px;
    background-color: var(--color-foreground-level-5);
    top: -4px;
    left: -16px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator.active:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 44px;
    background-color: var(--color-secondary);
    top: -4px;
    left: -16px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .wrapper :global(li:hover svg) {
    fill: var(--color-secondary);
  }

  .indicator.active :global(svg) {
    fill: var(--color-secondary);
  }

  .item:hover .tooltip {
    opacity: 1;
    transition: 0.3s;
  }

  .tooltip {
    white-space: nowrap;
    user-select: none;
    background-color: var(--color-foreground);
    color: var(--color-background);
    text-align: center;
    border-radius: 4px;
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
    border-right: 6px solid var(--color-foreground);
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
  <ul class="top">
    <li
      class="item indicator"
      data-cy="profile"
      class:active={path.active(path.profile(), $location, true)}>
      <a href={path.profileProjects()} use:link>
        <Avatar
          size="medium"
          avatarFallback={identity.avatarFallback}
          imageUrl={identity.metadata.avatarUrl}
          variant="circle" />
      </a>

      <div class="tooltip">
        <Title>Profile</Title>
      </div>
    </li>

    {#each orgs as org}
      <li
        class="item indicator"
        data-cy={`org-${org.id}`}
        class:active={path.active(path.orgs(org.id), $location, true)}>
        <a href={path.orgProjects(org.id)} use:link>
          <Avatar
            avatarFallback={org.avatarFallback}
            variant="square"
            size="medium" />
        </a>

        <div class="tooltip">
          <Title>{org.id}</Title>
        </div>
      </li>
    {/each}

    <li class="item" data-cy="add-org-button">
      <AddOrgButton on:click={() => dispatch('createorg')} />
      <div class="tooltip">
        <Title>Add org</Title>
      </div>
    </li>
  </ul>
  <ul class="bottom">
    <li class="item indicator" data-cy="wallet">
      <a href={path.profileWallet()} use:link>
        <Icon.Fund />
      </a>

      <div class="tooltip">
        <Title style="white-space: nowrap;">Wallet</Title>
      </div>
    </li>
    <li
      class="item indicator"
      data-cy="settings"
      class:active={path.active(path.settings(), $location)}>
      <a href={path.settings()} use:link>
        <Icon.Settings />
      </a>

      <div class="tooltip">
        <Title style="white-space: nowrap;">Settings</Title>
      </div>
    </li>
  </ul>
</div>
