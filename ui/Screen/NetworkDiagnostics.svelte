<script lang="typescript">
  import * as router from "ui/src/router";
  import { status as store } from "ui/src/localPeer";

  import { Icon } from "ui/DesignSystem/Primitive";
  import {
    ActionBar,
    Header,
    TabBar,
    SidebarLayout,
  } from "ui/DesignSystem/Component";

  import ConnectedPeersTab from "ui/Screen/NetworkDiagnostics/ConnectedPeers.svelte";
  import WaitingRoomTab from "ui/Screen/NetworkDiagnostics/WaitingRoom.svelte";

  export let activeTab: router.NetworkDiagnosticsTab = "peers";

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
  .title {
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
</style>

<SidebarLayout>
  <Header>
    <div slot="left" class="title">
      <h1>Status: {$store.type}</h1>
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(activeTab)} />
    </div>
  </ActionBar>

  {#if activeTab === "peers"}
    <ConnectedPeersTab />
  {:else if activeTab === "requests"}
    <WaitingRoomTab />
  {:else}
    {router.unreachable(activeTab)}
  {/if}
</SidebarLayout>
