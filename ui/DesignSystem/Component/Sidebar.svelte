<script>
  import { createEventDispatcher } from "svelte";
  import { location, link } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import Tooltip from "./Tooltip.svelte";
  import { Avatar, Icon } from "../Primitive";

  import AddOrgButton from "./Sidebar/AddOrgButton.svelte";

  const dispatch = createEventDispatcher();

  export let identity = null;
  export let orgs = null;
  export let registerOrgPermission = null;
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
    padding-bottom: 32px;
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
    height: 32px;
    width: var(--sidebar-width);
    top: -32px;
    left: 0;
    background: linear-gradient(
      0deg,
      var(--color-foreground-level-2) 0%,
      rgba(0, 0, 0, 0) 100%
    );
  }

  .item {
    width: var(--sidebar-width);
    height: 32px;
    margin-bottom: 16px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .indicator:hover:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 32px;
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
    height: 32px;
    background-color: var(--color-secondary);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator :global(li:hover svg) {
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
            size="regular"
            avatarFallback={identity.avatarFallback}
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
              size="regular" />
          </a>
        </Tooltip>
      </li>
    {/each}

    <li class="item" data-cy="add-org">
      {#if registerOrgPermission}
        <Tooltip value="Add org">
          <AddOrgButton on:click={() => dispatch('createorg')} />
        </Tooltip>
      {:else}
        <Tooltip value="Register your handle to create an org">
          <AddOrgButton disabled={true} />
        </Tooltip>
      {/if}
    </li>
  </ul>
  <ul class="bottom">
    <!-- TODO(sos): make active when search is open -->
    <li class="item indicator" data-cy="search">
      <Tooltip value="Navigate to a project">
        <div on:click|stopPropagation={() => dispatch('opensearch')}>
          <Icon.Search />
        </div>
      </Tooltip>
    </li>
    <li
      class="item indicator"
      class:active={path.active(path.discovery(), $location)}
      data-cy="discovery">
      <Tooltip value="Discover">
        <a href={path.discovery()} use:link>
          <Icon.Peer />
        </a>
      </Tooltip>
    </li>
    <li
      class="item indicator"
      data-cy="wallet"
      class:active={path.active(path.profileWallet(), $location)}>
      <Tooltip value="Wallet">
        <a href={path.profileWallet()} use:link>
          <Icon.Fund />
        </a>
      </Tooltip>
    </li>
    <li
      class="item indicator"
      data-cy="settings"
      class:active={path.active(path.settings(), $location)}>
      <Tooltip value="Settings">
        <a href={path.settings()} use:link>
          <Icon.Settings />
        </a>
      </Tooltip>
    </li>
  </ul>
</div>
