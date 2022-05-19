<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { unreachable } from "ui/src/unreachable";

  import { notificationHistory } from "ui/src/notification";
  import { status as localPeerState } from "ui/src/localPeer";
  import { waitingRoomEventLog, waitingRoomState } from "ui/src/localPeer";
  import * as Session from "ui/src/session";
  import * as ipc from "ui/src/ipc";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";

  import Button from "design-system/Button.svelte";

  import CopyIcon from "design-system/icons/Copy.svelte";
  import FileIcon from "design-system/icons/File.svelte";
  import FolderIcon from "design-system/icons/Folder.svelte";
  import InfoCircleIcon from "design-system/icons/InfoCircle.svelte";
  import NetworkIcon from "design-system/icons/Network.svelte";
  import TransactionsIcon from "design-system/icons/Transactions.svelte";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ConnectionsTab from "./DiagnosticsScreen/Connections.svelte";
  import NotificationHistoryTab from "./DiagnosticsScreen/NotificationHistory.svelte";
  import ProxyLogsTab from "./DiagnosticsScreen/ProxyLogs.svelte";
  import StorageTab from "./DiagnosticsScreen/Storage.svelte";
  import WaitingRoomTab from "./DiagnosticsScreen/WaitingRoom.svelte";

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
    {
      title: "Notification History",
      active: active === "notificationHistory",
      icon: InfoCircleIcon,
      onClick: () => {
        router.push({ type: "diagnostics", activeTab: "notificationHistory" });
      },
    },
    {
      title: "Proxy Logs",
      active: active === "proxyLogs",
      icon: FileIcon,
      onClick: () => {
        router.push({ type: "diagnostics", activeTab: "proxyLogs" });
      },
    },
  ];

  async function copyEverythingToClipboard() {
    const diagnostics = await proxy.client.diagnosticsGet();
    ipc.copyToClipboard(
      JSON.stringify(
        {
          upstreamVersion: await ipc.getVersion(),
          storage: diagnostics.storage,
          p2pConnections: {
            yourIdentity: Session.unsealed().identity,
            connectionStatus: $localPeerState,
            yourPeer: diagnostics.peer,
          },
          waitingRoom: {
            latestState: $waitingRoomState,
            stateTransitions: $waitingRoomEventLog,
          },
          notificationHistory: $notificationHistory,
          proxyLogs: (await ipc.getProxyLogs()).split("\n"),
        },
        null,
        2
      )
    );
    notification.show({
      type: "info",
      message: "Copied all debug info to clipboard",
    });
  }
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    padding: 1rem var(--content-padding) 2rem var(--content-padding);
  }
  h1 {
    margin-bottom: 0.5rem;
  }
  .version {
    color: var(--color-foreground-level-6);
  }

  .copy-button {
    display: flex;
    align-self: center;
  }
</style>

<ScreenLayout contentStyle="padding: 0;">
  <div slot="header" style="display: flex;">
    <div style="width: -webkit-fill-available;">
      <h1>Diagnostics</h1>
      {#await ipc.getVersion() then version}
        <span class="version">Upstream version: {version}</span>
      {/await}
    </div>
    <div class="copy-button">
      <Button
        variant="outline"
        icon={CopyIcon}
        style="margin-left: auto; align-self: center"
        on:click={copyEverythingToClipboard}>
        Copy all debug info to clipboard
      </Button>
    </div>
  </div>

  <ActionBar>
    <TabBar tabs={tabs(activeTab)} />
  </ActionBar>

  <div class="container">
    {#if activeTab === "connections"}
      <ConnectionsTab />
    {:else if activeTab === "storage"}
      <StorageTab />
    {:else if activeTab === "waitingRoom"}
      <WaitingRoomTab />
    {:else if activeTab === "notificationHistory"}
      <NotificationHistoryTab />
    {:else if activeTab === "proxyLogs"}
      <ProxyLogsTab />
    {:else}
      {unreachable(activeTab)}
    {/if}
  </div>
</ScreenLayout>
