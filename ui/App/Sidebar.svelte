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

  import MagnifyingGlassIcon from "design-system/icons/MagnifyingGlass.svelte";
  import NetworkIcon from "design-system/icons/Network.svelte";
  import SettingsIcon from "design-system/icons/Settings.svelte";

  import Avatar from "design-system/Avatar.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import SearchModal from "ui/App/SearchModal.svelte";

  import SidebarItem from "./Sidebar/SidebarItem.svelte";

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
  </div>
  <div class="bottom">
    <Tooltip value="Navigate to a project">
      <SidebarItem
        dataCy="search"
        indicator
        onClick={() => modal.toggle(SearchModal)}>
        <MagnifyingGlassIcon />
      </SidebarItem>
    </Tooltip>
    <Tooltip value="Network">
      <SidebarItem
        dataCy="network"
        indicator
        active={$activeRouteStore.type === "network"}
        onClick={() => push({ type: "network" })}>
        <NetworkIcon />
      </SidebarItem>
    </Tooltip>
    <Tooltip value="Settings">
      <SidebarItem
        dataCy="settings"
        indicator
        active={$activeRouteStore.type === "settings"}
        onClick={() => push({ type: "settings" })}>
        <SettingsIcon />
      </SidebarItem>
    </Tooltip>
  </div>
</div>
