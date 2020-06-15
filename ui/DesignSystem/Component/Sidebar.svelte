<script>
  import { createEventDispatcher } from "svelte";
  import { location, link } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Tooltip } from "../Component";
  import { Avatar, Icon } from "../Primitive";

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
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
  }

  .top {
    overflow-y: scroll;
    padding-bottom: 36px;
    padding-top: 16px;
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
    width: 68px;
    top: -36px;
    left: 0;
    background: linear-gradient(
      0deg,
      rgba(235, 239, 243, 1) 0%,
      rgba(235, 239, 243, 0) 100%
    );
  }

  .item {
    width: 68px;
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
    height: 36px;
    background-color: var(--color-foreground-level-5);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator.active:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 36px;
    background-color: var(--color-secondary);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .wrapper :global(li:hover svg) {
    fill: var(--color-secondary);
  }

  .indicator.active :global(svg) {
    fill: var(--color-secondary);
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
      <Tooltip value={identity.metadata.handle}>
        <a href={path.profileProjects()} use:link>
          <Avatar
            size="medium"
            avatarFallback={identity.avatarFallback}
            imageUrl={identity.metadata.avatarUrl}
            variant="circle" />
        </a>
      </Tooltip>
    </li>

    {#each orgs as org}
      <li
        class="item indicator"
        data-cy={`org-${org.id}`}
        class:active={path.active(path.orgs(org.id), $location, true)}>
        <Tooltip value={org.id}>
          <a href={path.orgProjects(org.id)} use:link>
            <Avatar
              avatarFallback={org.avatarFallback}
              variant="square"
              size="medium" />
          </a>
        </Tooltip>
      </li>
    {/each}

    <li class="item" data-cy="add-org-button">
      <Tooltip value="Add org">
        <AddOrgButton on:click={() => dispatch('createorg')} />
      </Tooltip>
    </li>
  </ul>
  <ul class="bottom">
    <Tooltip value="Wallet">
      <li class="item indicator" data-cy="wallet">
        <a href={path.profileWallet()} use:link>
          <Icon.Fund />
        </a>
      </li>
    </Tooltip>
    <Tooltip value="Settings">
      <li
        class="item indicator"
        data-cy="settings"
        class:active={path.active(path.settings(), $location)}>
        <a href={path.settings()} use:link>
          <Icon.Settings />
        </a>
      </li>
    </Tooltip>
  </ul>
</div>
