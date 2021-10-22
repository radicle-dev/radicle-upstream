<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
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

  import Hotkeys from "ui/App/Hotkeys.svelte";
  import ModalLayout from "ui/App/ModalLayout.svelte";
  import NotificationFaucet from "ui/App/NotificationFaucet.svelte";
  import Theme from "ui/App/Theme.svelte";

  import DesignSystemGuideModal from "ui/App/DesignSystemGuideModal.svelte";
  import OnboardingModal from "ui/App/OnboardingModal.svelte";

  import LoadingScreen from "ui/App/LoadingScreen.svelte";
  import LockScreen from "ui/App/LockScreen.svelte";
  import NetworkDiagnosticsScreen from "ui/App/NetworkDiagnosticsScreen.svelte";
  import NetworkScreen from "ui/App/NetworkScreen.svelte";
  import OrgsScreen from "ui/App/OrgsScreen.svelte";
  import OrgScreen from "ui/App/OrgScreen.svelte";
  import ProfileScreen from "ui/App/ProfileScreen.svelte";
  import ProjectScreen from "ui/App/ProjectScreen.svelte";
  import SettingsScreen from "ui/App/SettingsScreen.svelte";
  import SingleSigOrgScreen from "ui/App/SingleSigOrgScreen.svelte";
  import UnrecoverableErrorScreen from "ui/App/UnrecoverableErrorScreen.svelte";
  import UserProfileScreen from "ui/App/UserProfileScreen.svelte";
  import WalletScreen from "ui/App/WalletScreen.svelte";

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
            router.push({ type: "profile" });
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

<UnrecoverableErrorScreen />
<Hotkeys />
<ModalLayout />
<NotificationFaucet />
<Theme />

{#if $sessionStore.status === remote.Status.Success}
  {#if $activeRouteStore.type === "designSystemGuide"}
    <DesignSystemGuideModal />
  {:else if $activeRouteStore.type === "lock"}
    <LockScreen />
  {:else if $activeRouteStore.type === "onboarding"}
    <OnboardingModal />
  {:else if $activeRouteStore.type === "profile"}
    <ProfileScreen />
  {:else if $activeRouteStore.type === "userProfile"}
    <UserProfileScreen
      ownUserUrn={$activeRouteStore.ownUserUrn}
      user={$activeRouteStore.user}
      projects={$activeRouteStore.projects} />
  {:else if $activeRouteStore.type === "networkDiagnostics"}
    <NetworkDiagnosticsScreen activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "singleSigOrg"}
    <SingleSigOrgScreen
      registration={$activeRouteStore.registration}
      address={$activeRouteStore.address}
      owner={$activeRouteStore.owner}
      projectCount={$activeRouteStore.projectCount}
      showWriteActions={$activeRouteStore.showWriteActions}
      anchors={$activeRouteStore.anchors} />
  {:else if $activeRouteStore.type === "multiSigOrg"}
    <OrgScreen
      registration={$activeRouteStore.registration}
      activeTab={$activeRouteStore.view}
      address={$activeRouteStore.address}
      gnosisSafeAddress={$activeRouteStore.gnosisSafeAddress}
      threshold={$activeRouteStore.threshold}
      showWriteActions={$activeRouteStore.showWriteActions}
      memberCount={$activeRouteStore.memberCount} />
  {:else if $activeRouteStore.type === "project"}
    <ProjectScreen
      activeView={$activeRouteStore.activeView}
      urn={$activeRouteStore.urn}
      anchors={$activeRouteStore.anchors} />
  {:else if $activeRouteStore.type === "network"}
    <NetworkScreen />
  {:else if $activeRouteStore.type === "orgs"}
    <OrgsScreen />
  {:else if $activeRouteStore.type === "settings"}
    <SettingsScreen />
  {:else if $activeRouteStore.type === "wallet"}
    <WalletScreen activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "boot"}
    <LoadingScreen />
  {:else}
    {unreachable($activeRouteStore)}
  {/if}
{:else}
  <LoadingScreen />
{/if}
