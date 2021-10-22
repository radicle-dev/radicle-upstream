<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { activeRouteStore, push } from "ui/src/router";
  import * as modal from "ui/src/modal";
  import * as Session from "ui/src/session";

  import { Avatar, Icon, Tooltip } from "ui/DesignSystem";

  import SearchModal from "ui/App/SearchModal.svelte";

  import ConnectionStatusIndicator from "./Sidebar/ConnectionStatusIndicator.svelte";
  import OrgList from "./Sidebar/OrgList.svelte";
  import SidebarItem from "./Sidebar/SidebarItem.svelte";
  import WalletStatusIndicator from "./Sidebar/WalletStatusIndicator.svelte";

  const session = Session.unsealed();
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
    <Tooltip value={session.identity.metadata.handle}>
      <SidebarItem
        dataCy="profile"
        indicator
        active={$activeRouteStore.type === "profile"}
        onClick={() => push({ type: "profile" })}>
        <Avatar
          size="regular"
          kind={{
            type: "userEmoji",
            uniqueIdentifier: session.identity.urn,
          }} />
      </SidebarItem>
    </Tooltip>
    <OrgList identity={session.identity} />
  </div>
  <div class="bottom">
    <Tooltip value="Navigate to a project">
      <SidebarItem
        dataCy="search"
        indicator
        onClick={() => modal.toggle(SearchModal)}>
        <Icon.MagnifyingGlass />
      </SidebarItem>
    </Tooltip>
    <Tooltip value="Orgs">
      <SidebarItem
        indicator
        active={$activeRouteStore.type === "orgs"}
        onClick={() => push({ type: "orgs" })}>
        <Icon.Orgs />
      </SidebarItem>
    </Tooltip>
    <WalletStatusIndicator
      active={$activeRouteStore.type === "wallet"}
      onClick={() => push({ type: "wallet", activeTab: "transactions" })} />
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
