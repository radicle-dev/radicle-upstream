<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as Session from "ui/src/session";
  import * as customProtocolHandler from "ui/src/customProtocolHandler";
  import * as error from "ui/src/error";
  import * as ethereum from "ui/src/ethereum";
  import * as hotkeys from "ui/src/hotkeys";
  import * as org from "./src/org";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import * as transaction from "ui/src/transaction";
  import * as walletModule from "ui/src/wallet";

  import { unreachable } from "ui/src/unreachable";
  import "ui/src/localPeer";

  import { NotificationFaucet, ModalOverlay } from "ui/DesignSystem";

  import Bsod from "ui/App/Bsod.svelte";
  import DesignSystemGuide from "ui/App/DesignSystemGuide.svelte";
  import Hotkeys from "ui/App/Hotkeys.svelte";
  import Loading from "ui/App/Loading.svelte";
  import Lock from "ui/App/Lock.svelte";
  import Network from "ui/App/Network.svelte";
  import NetworkDiagnostics from "ui/App/NetworkDiagnostics.svelte";
  import Onboarding from "ui/App/Onboarding.svelte";
  import Org from "ui/App/Org.svelte";
  import Profile from "ui/App/Profile.svelte";
  import Project from "ui/App/Project.svelte";
  import Settings from "ui/App/Settings.svelte";
  import SingleSigOrg from "ui/App/SingleSigOrg.svelte";
  import Theme from "ui/App/Theme.svelte";
  import UserProfile from "ui/App/UserProfile.svelte";
  import Wallet from "ui/App/Wallet.svelte";

  router.initialize();
  customProtocolHandler.register();
  org.initialize();
  transaction.initialize();

  const activeRouteStore = router.activeRouteStore;
  const ethereumEnvironment = ethereum.selectedEnvironment;
  const sessionStore = Session.session;
  const walletStore = walletModule.store;

  sessionStore.subscribe(session => {
    // We’re not using a reactive statement here to prevent this code from
    // running when `activeRouteStore` is updated.
    switch (session.status) {
      case remote.Status.NotAsked:
        Session.fetch();
        break;

      case remote.Status.Success:
        if (session.data.status === Session.Status.NoSession) {
          hotkeys.disable();
          router.push({ type: "onboarding" });
        } else if (session.data.status === Session.Status.SealedSession) {
          hotkeys.disable();
          router.push({ type: "lock" });
        } else if (session.data.status === Session.Status.UnsealedSession) {
          hotkeys.enable();
          if (
            $activeRouteStore.type === "onboarding" ||
            $activeRouteStore.type === "lock" ||
            $activeRouteStore.type === "boot"
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

  $: connectedNetwork = ethereum.supportedNetwork($ethereumEnvironment);
  $: wallet = $walletStore;
  $: walletState = $wallet;

  // If we're on an org screen and there's a wallet mismatch, go to the wallet
  // screen to inform the user about the mismatch.
  $: if (
    walletState.status === walletModule.Status.Connected &&
    connectedNetwork !== walletState.connected.network &&
    ($activeRouteStore.type === "singleSigOrg" ||
      $activeRouteStore.type === "multiSigOrg")
  ) {
    router.push({ type: "wallet", activeTab: "transactions" });
  }
</script>

<Bsod />
<Hotkeys />
<ModalOverlay />
<NotificationFaucet />
<Theme />

{#if $sessionStore.status === remote.Status.Success}
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
  {:else if $activeRouteStore.type === "singleSigOrg"}
    <SingleSigOrg
      registration={$activeRouteStore.registration}
      address={$activeRouteStore.address}
      owner={$activeRouteStore.owner}
      projectCount={$activeRouteStore.projectCount}
      anchors={$activeRouteStore.anchors} />
  {:else if $activeRouteStore.type === "multiSigOrg"}
    <Org
      registration={$activeRouteStore.registration}
      activeTab={$activeRouteStore.view}
      address={$activeRouteStore.address}
      gnosisSafeAddress={$activeRouteStore.gnosisSafeAddress}
      threshold={$activeRouteStore.threshold}
      members={$activeRouteStore.members} />
  {:else if $activeRouteStore.type === "project"}
    <Project
      activeView={$activeRouteStore.activeView}
      urn={$activeRouteStore.urn} />
  {:else if $activeRouteStore.type === "network"}
    <Network />
  {:else if $activeRouteStore.type === "settings"}
    <Settings />
  {:else if $activeRouteStore.type === "wallet"}
    <Wallet activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "boot"}
    <Loading />
  {:else}
    {unreachable($activeRouteStore)}
  {/if}
{:else}
  <Loading />
{/if}
