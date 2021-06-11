<script lang="typescript">
  import * as router from "ui/src/router";
  import * as customProtocolHandler from "ui/src/customProtocolHandler";
  import * as error from "ui/src/error";
  import * as org from "./src/org";
  import * as wallet from "./src/wallet";
  import * as hotkeys from "ui/src/hotkeys";
  import * as remote from "ui/src/remote";
  import { unreachable } from "ui/src/unreachable";
  import { fetch, session as sessionStore, Status } from "ui/src/session";
  import "ui/src/localPeer";

  import {
    EmptyState,
    NotificationFaucet,
    ModalOverlay,
    Remote,
  } from "ui/DesignSystem/Component";

  import Hotkeys from "ui/Hotkeys.svelte";
  import Theme from "ui/Theme.svelte";

  import Bsod from "ui/Screen/Bsod.svelte";
  import DesignSystemGuide from "ui/Screen/DesignSystemGuide.svelte";
  import Lock from "ui/Screen/Lock.svelte";
  import NetworkDiagnostics from "ui/Screen/NetworkDiagnostics.svelte";
  import Onboarding from "ui/Screen/Onboarding.svelte";
  import Org from "ui/Screen/Org.svelte";
  import Profile from "ui/Screen/Profile.svelte";
  import UserProfile from "ui/Screen/UserProfile.svelte";
  import Project from "ui/Screen/Project.svelte";
  import Settings from "ui/Screen/Settings.svelte";
  import Wallet from "ui/Screen/Wallet.svelte";

  router.initialize();
  customProtocolHandler.register();

  const activeRouteStore = router.activeRouteStore;

  sessionStore.subscribe(session => {
    // We’re not using a reactive statement here to prevent this code from
    // running when `activeRouteStore` is updated.
    switch (session.status) {
      case remote.Status.NotAsked:
        fetch();
        break;

      case remote.Status.Success:
        if (session.data.status === Status.NoSession) {
          hotkeys.disable();
          router.push({ type: "onboarding" });
        } else if (session.data.status === Status.SealedSession) {
          hotkeys.disable();
          router.push({ type: "lock" });
        } else if (session.data.status === Status.UnsealedSession) {
          hotkeys.enable();
          if (
            $activeRouteStore.type === "onboarding" ||
            $activeRouteStore.type === "lock"
          ) {
            router.push({ type: "profile", activeTab: "projects" });
          }
        } else {
          unreachable(session.data);
        }
        break;

      case remote.Status.Error:
        error.show(session.error);
        break;
    }
  });

  const walletStore = wallet.store;
  $: w = $walletStore;

  $: if ($w.status === wallet.Status.Connected) {
    (async () => {
      await org.fetchOrgs();
    })();
  }
</script>

<style>
  .error {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<Bsod />
<Hotkeys />
<ModalOverlay />
<NotificationFaucet />
<Theme />

<Remote store={sessionStore} context="session" disableErrorLogging={true}>
  {#if $activeRouteStore.type === "designSystemGuide"}
    <DesignSystemGuide />
  {:else if $activeRouteStore.type === "lock"}
    <Lock />
  {:else if $activeRouteStore.type === "onboarding"}
    <Onboarding />
  {:else if $activeRouteStore.type === "profile"}
    <Profile activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "userProfile"}
    <UserProfile urn={$activeRouteStore.urn} />
  {:else if $activeRouteStore.type === "networkDiagnostics"}
    <NetworkDiagnostics activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "org"}
    <Org
      activeTab={$activeRouteStore.activeTab}
      address={$activeRouteStore.address}
      gnosisSafeAddress={$activeRouteStore.gnosisSafeAddress}
      threshold={$activeRouteStore.threshold}
      members={$activeRouteStore.members} />
  {:else if $activeRouteStore.type === "project"}
    <Project
      activeView={$activeRouteStore.activeView}
      urn={$activeRouteStore.urn} />
  {:else if $activeRouteStore.type === "settings"}
    <Settings />
  {:else if $activeRouteStore.type === "wallet"}
    <Wallet activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "boot"}
    <!-- TODO: show some loading screen -->
  {:else}
    {router.unreachable($activeRouteStore)}
  {/if}

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
