<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as router from "ui/src/router";
  import { unreachable } from "ui/src/unreachable";
  import { status as store } from "ui/src/localPeer";

  import { Icon } from "ui/DesignSystem";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ConnectedPeersTab from "./NetworkDiagnosticsScreen/ConnectedPeers.svelte";
  import WaitingRoomTab from "./NetworkDiagnosticsScreen/WaitingRoom.svelte";

  export let activeTab: router.NetworkDiagnosticsTab;

  const tabs = (active: router.NetworkDiagnosticsTab) => [
    {
      title: "Peers",
      active: active === "peers",
      icon: Icon.Network,
      onClick: () => {
        router.push({ type: "networkDiagnostics", activeTab: "peers" });
      },
    },
    {
      title: "Requests",
      active: active === "requests",
      icon: Icon.Road,
      onClick: () => {
        router.push({ type: "networkDiagnostics", activeTab: "requests" });
      },
    },
  ];
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    margin: 0 auto;
    padding: 2rem var(--content-padding);
  }
</style>

<ScreenLayout>
  <h1 slot="header">Status: {$store.type}</h1>
  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(activeTab)} />
    </div>
  </ActionBar>
  <div class="container">
    {#if activeTab === "peers"}
      <ConnectedPeersTab />
    {:else if activeTab === "requests"}
      <WaitingRoomTab />
    {:else}
      {unreachable(activeTab)}
    {/if}
  </div>
</ScreenLayout>
