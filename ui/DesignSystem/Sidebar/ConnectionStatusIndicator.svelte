<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { activeRouteStore, push } from "ui/src/router";
  import { status, StatusType } from "ui/src/localPeer";

  import Icon from "ui/DesignSystem/Icon";
  import Tooltip from "ui/DesignSystem/Tooltip.svelte";
  import SidebarItem from "ui/DesignSystem/Sidebar/SidebarItem.svelte";

  import Syncing from "./ConnectionStatusIndicator/Syncing.svelte";
  import Offline from "./ConnectionStatusIndicator/Offline.svelte";

  const connectedPeerCount = (peers: {
    [peerId: string]: string[];
  }): string => {
    const count = Object.keys(peers).length;
    return peerCount(count);
  };

  const peerCount = (count: number) => {
    if (count === 1) {
      return "1 peer";
    } else {
      return `${count} peers`;
    }
  };
</script>

<div>
  {#if $status.type === StatusType.Online}
    <Tooltip
      value={`Network • You’re connected to ${connectedPeerCount(
        $status.connectedPeers
      )}`}>
      <SidebarItem
        dataCy="connection-status-online"
        indicator
        active={$activeRouteStore.type === "network"}
        onClick={() => push({ type: "network" })}>
        <Icon.Network />
      </SidebarItem>
    </Tooltip>
  {:else if $status.type === StatusType.Syncing}
    <Tooltip
      value={`Network • Syncing with ${peerCount(
        $status.syncs
      )} to get new content from your network`}>
      <SidebarItem
        dataCy="connection-status-syncing"
        indicator
        active={$activeRouteStore.type === "network"}
        onClick={() => push({ type: "network" })}>
        <Syncing />
      </SidebarItem>
    </Tooltip>
  {:else if $status.type === StatusType.Offline || $status.type === StatusType.Started}
    <Tooltip value="Network • You’re not connected to any peers">
      <SidebarItem
        dataCy="connection-status-offline"
        indicator
        active={$activeRouteStore.type === "network"}
        onClick={() => push({ type: "network" })}>
        <Offline />
      </SidebarItem>
    </Tooltip>
  {:else if $status.type === StatusType.Stopped}
    <Tooltip value="Network • The app couldn't start your peer">
      <SidebarItem
        dataCy="connection-status-stopped"
        indicator
        active={$activeRouteStore.type === "network"}
        onClick={() => push({ type: "network" })}>
        <Offline style="fill: var(--color-negative);" />
      </SidebarItem>
    </Tooltip>
  {/if}
</div>
