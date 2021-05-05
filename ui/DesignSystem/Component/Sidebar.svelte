<script lang="typescript">
  import { location, push } from "svelte-spa-router";

  import type { Identity } from "../../src/identity";
  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { orgSidebarStore } from "../../src/org";

  import Tooltip from "./Tooltip.svelte";
  import { Avatar, Icon } from "../Primitive";
  import ConnectionStatusIndicator from "./ConnectionStatusIndicator.svelte";
  import AddOrgButton from "./Sidebar/AddOrgButton.svelte";
  import ModalSearch from "../../Modal/Search.svelte";
  import ModalCreateOrg from "../../Modal/Org/Create.svelte";

  export let identity: Identity;
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
    cursor: pointer;
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
    background-color: var(--color-primary);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator :global(div:hover svg) {
    fill: var(--color-primary);
  }

  .indicator.active :global(svg) {
    fill: var(--color-primary);
  }
</style>

<div class="wrapper" data-cy="sidebar">
  <div class="top">
    <Tooltip value={identity.metadata.handle}>
      <div
        class="item indicator"
        data-cy="profile"
        class:active={$location.startsWith(path.profile())}
        on:click|stopPropagation={() => push(path.profileProjects())}>
        <Avatar
          size="regular"
          avatarFallback={identity.avatarFallback}
          variant="circle" />
      </div>
    </Tooltip>
    {#each $orgSidebarStore as org (org.id)}
      <Tooltip value={org.id}>
        <div
          class="item indicator"
          class:active={$location.startsWith(path.org(org.id))}
          on:click|stopPropagation={() => push(path.orgProjects(org.id))}>
          <Avatar size="regular" variant="square" />
        </div>
      </Tooltip>
    {/each}
    <Tooltip value="Create an org">
      <div
        class="item indicator"
        data-cy="add-org-btn"
        on:click|stopPropagation={() => modal.toggle(ModalCreateOrg, () => {}, {
            identity,
          })}>
        <AddOrgButton />
      </div>
    </Tooltip>
  </div>
  <div class="bottom">
    <Tooltip value="Navigate to a project">
      <div
        class="item indicator"
        data-cy="search"
        on:click|stopPropagation={() => modal.toggle(ModalSearch)}>
        <Icon.MagnifyingGlass />
      </div>
    </Tooltip>
    <ConnectionStatusIndicator />
    <Tooltip value="Settings">
      <div
        class="item indicator"
        data-cy="settings"
        class:active={$location.startsWith(path.settings())}
        on:click|stopPropagation={() => push(path.settings())}>
        <Icon.Settings />
      </div>
    </Tooltip>
  </div>
</div>
