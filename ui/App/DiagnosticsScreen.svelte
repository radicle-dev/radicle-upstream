<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { unreachable } from "ui/src/unreachable";

  import { status as localPeerState } from "ui/src/localPeer";
  import { waitingRoomEventLog, waitingRoomState } from "ui/src/localPeer";
  import * as Session from "ui/src/session";
  import * as ipc from "ui/src/ipc";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";

  import Button from "design-system/Button.svelte";

  import CopyIcon from "design-system/icons/Copy.svelte";
  import FolderIcon from "design-system/icons/Folder.svelte";
  import NetworkIcon from "design-system/icons/Network.svelte";
  import TransactionsIcon from "design-system/icons/Transactions.svelte";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ConnectionsTab from "./DiagnosticsScreen/Connections.svelte";
  import WaitingRoomTab from "./DiagnosticsScreen/WaitingRoom.svelte";
  import StorageTab from "./DiagnosticsScreen/Storage.svelte";

  export let activeTab: router.DiagnosticsTab;

  const tabs = (active: router.DiagnosticsTab) => [
    {
      title: "Storage",
      active: active === "storage",
      icon: FolderIcon,
      onClick: () => {
        router.push({ type: "diagnostics", activeTab: "storage" });
      },
    },
    {
      title: "P2P Connections",
      active: active === "connections",
      icon: NetworkIcon,
      onClick: () => {
        router.push({ type: "diagnostics", activeTab: "connections" });
      },
    },
    {
      title: "Waiting Room",
      active: active === "waitingRoom",
      icon: TransactionsIcon,
      onClick: () => {
        router.push({ type: "diagnostics", activeTab: "waitingRoom" });
      },
    },
  ];

  async function copyEverythingToClipboard() {
    const diagnostics = await proxy.client.diagnosticsGet();
    ipc.copyToClipboard(
      JSON.stringify(
        {
          storage: diagnostics.storage,
          identity: Session.unsealed().identity,
          localPeer: {
            membership: diagnostics.peer.membership,
            state: $localPeerState,
          },
          waitingRoomState: $waitingRoomState,
          waitingRoomEventLog: $waitingRoomEventLog,
        },
        null,
        2
      )
    );
    notification.info({ message: "Copied all debug info to clipboard" });
  }
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    padding: 1rem var(--content-padding) 2rem var(--content-padding);
  }
</style>

<ScreenLayout contentStyle="padding: 0;">
  <div slot="header" style="display: flex;">
    <h1>Diagnostics</h1>
    <Button
      variant="outline"
      icon={CopyIcon}
      style="margin-left: auto; align-self: center"
      on:click={copyEverythingToClipboard}>
      Copy all debug info to clipboard
    </Button>
  </div>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(activeTab)} />
    </div>
  </ActionBar>

  <div class="container">
    {#if activeTab === "connections"}
      <ConnectionsTab />
    {:else if activeTab === "storage"}
      <StorageTab />
    {:else if activeTab === "waitingRoom"}
      <WaitingRoomTab />
    {:else}
      {unreachable(activeTab)}
    {/if}
  </div>
</ScreenLayout>
