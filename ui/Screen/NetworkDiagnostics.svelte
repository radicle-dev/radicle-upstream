<script lang="typescript">
  import Router from "svelte-spa-router";

  import { status as store } from "ui/src/localPeer";
  import * as path from "ui/src/path";

  import ActionBar from "ui/DesignSystem/Component/ActionBar.svelte";
  import Header from "ui/DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "ui/DesignSystem/Component/HorizontalMenu.svelte";
  import SidebarLayout from "ui/DesignSystem/Component/SidebarLayout.svelte";

  import IconNetwork from "ui/DesignSystem/Primitive/Icon/Network.svelte";
  import IconRoad from "ui/DesignSystem/Primitive/Icon/Road.svelte";

  import ConnectedPeers from "./NetworkDiagnostics/ConnectedPeers.svelte";
  import WaitingRoom from "./NetworkDiagnostics/WaitingRoom.svelte";

  const screenRoutes = {
    [path.networkDiagnosticsConnectedPeers()]: ConnectedPeers,
    [path.networkDiagnosticsWaitingRoom()]: WaitingRoom,
  };

  const topbarMenuItems = [
    {
      icon: IconNetwork,
      title: "Peers",
      href: path.networkDiagnosticsConnectedPeers(),
      looseActiveStateMatching: true,
    },
    {
      icon: IconRoad,
      title: "Requests",
      href: path.networkDiagnosticsWaitingRoom(),
      looseActiveStateMatching: true,
    },
  ];
</script>

<style>
  .container {
    margin: 0 auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

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
      <HorizontalMenu items={topbarMenuItems} />
    </div>
  </ActionBar>
  <div class="container">
    <Router routes={screenRoutes} />
  </div>
</SidebarLayout>
