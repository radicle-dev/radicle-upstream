<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import { activeRouteStore, push } from "ui/src/router";

  import { orgSidebarStore } from "ui/src/org";
  import type { Identity } from "ui/src/identity";
  import * as modal from "ui/src/modal";
  import * as config from "ui/src/config";
  import * as wallet from "ui/src/wallet";

  import Tooltip from "./Tooltip.svelte";
  import { Avatar, Icon } from "../Primitive";
  import SidebarItem from "./SidebarItem.svelte";
  import ConnectionStatusIndicator from "./ConnectionStatusIndicator.svelte";
  import AddOrgButton from "./Sidebar/AddOrgButton.svelte";
  import WalletStatusIndicator from "./WalletStatusIndicator.svelte";
  import ModalSearch from "ui/Modal/Search.svelte";
  import ModalCreateOrg from "../../Modal/Org/Create.svelte";

  export let identity: Identity;

  const walletStore = wallet.store;
  $: w = $walletStore;
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
        onClick={() => push({ type: "profile", activeTab: "projects" })}>
        <Avatar
          size="regular"
          avatarFallback={identity.avatarFallback}
          variant="circle" />
      </SidebarItem>
    </Tooltip>
    {#if $w.status === wallet.Status.Connected}
      {#each $orgSidebarStore as org (org.id)}
        <Tooltip value={org.id}>
          <SidebarItem
            indicator={true}
            onClick={() =>
              push({ type: "org", address: org.id, activeTab: "projects" })}
            active={$activeRouteStore.type === "org" &&
              $activeRouteStore.address === org.id}>
            <Avatar
              size="regular"
              variant="square"
              avatarFallback={radicleAvatar.generate(
                org.id,
                radicleAvatar.Usage.Any
              )} />
          </SidebarItem>
        </Tooltip>
      {/each}
      <Tooltip value="Create an org">
        <SidebarItem
          onClick={() =>
            modal.toggle(ModalCreateOrg, () => {}, {
              identity,
              walletAddress:
                $w.status === wallet.Status.Connected
                  ? $w.connected.account.address
                  : null,
            })}>
          <AddOrgButton />
        </SidebarItem>
      </Tooltip>
    {/if}
  </div>
  <div class="bottom">
    <Tooltip value="Navigate to a project">
      <SidebarItem dataCy="search" onClick={() => modal.toggle(ModalSearch)}>
        <Icon.MagnifyingGlass />
      </SidebarItem>
    </Tooltip>
    {#if config.isDev}
      <WalletStatusIndicator
        active={$activeRouteStore.type === "wallet"}
        onClick={() => push({ type: "wallet", activeTab: "transactions" })} />
    {/if}
    <ConnectionStatusIndicator />
    <Tooltip value="Settings">
      <SidebarItem
        dataCy="settings"
        indicator
        active={$activeRouteStore.type === "settings"}
        onClick={() => push({ type: "settings" })}>
        <Icon.Settings />
      </SidebarItem>
    </Tooltip>
  </div>
</div>
