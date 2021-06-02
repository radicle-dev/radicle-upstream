<script lang="typescript">
  import { activeRouteStore, push } from "ui/src/router";

  import type { Identity } from "ui/src/identity";
  import * as modal from "ui/src/modal";
  import * as config from "ui/src/config";

  import Tooltip from "./Tooltip.svelte";
  import { Avatar, Icon } from "../Primitive";
  import SidebarItem from "./SidebarItem.svelte";
  import ConnectionStatusIndicator from "./ConnectionStatusIndicator.svelte";
  import WalletStatusIndicator from "./WalletStatusIndicator.svelte";
  import ModalSearch from "ui/Modal/Search.svelte";

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
</style>

<div class="wrapper" data-cy="sidebar">
  <div class="top">
    <Tooltip value={identity.metadata.handle}>
      <SidebarItem
        dataCy="profile"
        indicator
        active={$activeRouteStore.type === "profile"}
        on:itemClick={() => push({ type: "profile", activeTab: "projects" })}>
        <Avatar
          size="regular"
          avatarFallback={identity.avatarFallback}
          variant="circle" />
      </SidebarItem>
    </Tooltip>
  </div>
  <div class="bottom">
    <Tooltip value="Navigate to a project">
      <SidebarItem
        dataCy="search"
        on:itemClick={() => modal.toggle(ModalSearch)}>
        <Icon.MagnifyingGlass />
      </SidebarItem>
    </Tooltip>
    {#if config.isDev}
      <WalletStatusIndicator
        active={$activeRouteStore.type === "wallet"}
        on:walletClick={() =>
          push({ type: "wallet", activeTab: "transactions" })} />
    {/if}
    <ConnectionStatusIndicator />
    <Tooltip value="Settings">
      <SidebarItem
        dataCy="settings"
        indicator
        active={$activeRouteStore.type === "settings"}
        on:itemClick={() => push({ type: "settings" })}>
        <Icon.Settings />
      </SidebarItem>
    </Tooltip>
  </div>
</div>
