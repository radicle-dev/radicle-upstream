<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Session from "ui/src/session";
  import * as config from "ui/src/config";
  import * as customProtocolHandler from "ui/src/customProtocolHandler";
  import * as error from "ui/src/error";
  import * as ethereum from "ui/src/ethereum";
  import * as hotkeys from "ui/src/hotkeys";
  import * as notification from "ui/src/notification";
  import * as org from "./src/org";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import * as transaction from "ui/src/transaction";
  import * as walletModule from "ui/src/wallet";

  import { unreachable } from "ui/src/unreachable";
  import "ui/src/localPeer";

  import Hotkeys from "ui/App/Hotkeys.svelte";
  import ModalLayout from "ui/App/ModalLayout.svelte";
  import Notifications from "ui/App/Notifications.svelte";
  import Theme from "ui/App/Theme.svelte";

  import DesignSystemShowcaseModal from "design-system/Showcase.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import SharedComponentShowcase from "ui/App/SharedComponents/SharedComponentShowcase.svelte";
  import CodeFontSetting from "ui/App/SharedComponents/CodeFontSetting.svelte";
  import PrimaryColorSetting from "ui/App/SharedComponents/PrimaryColorSetting.svelte";
  import ThemeSetting from "ui/App/SharedComponents/ThemeSetting.svelte";
  import UiFontSetting from "ui/App/SharedComponents/UiFontSetting.svelte";

  import OnboardingModal from "ui/App/OnboardingModal.svelte";

  import LockScreen from "ui/App/LockScreen.svelte";
  import DiagnosticsScreen from "ui/App/DiagnosticsScreen.svelte";
  import PushPullNetworkScreen from "ui/App/PushPullNetworkScreen.svelte";
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

  // If we’re not in any kind of test environment we show unhandled
  // errors to the user.
  if (
    !config.isCypressTestEnv &&
    !config.isCypressTestRunner &&
    !config.isNodeTestEnv
  ) {
    window.addEventListener("unhandledrejection", ev => {
      ev.preventDefault();
      notification.showException(
        error.fromUnknown(ev.reason, error.Code.UnhandledRejection)
      );
    });

    window.addEventListener("error", ev => {
      ev.preventDefault();
      notification.showException(
        error.fromUnknown(ev.error, error.Code.UnhandledError)
      );
    });
  }

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
          router.replace({ type: "onboarding" });
        } else if (session.data.status === Session.Status.SealedSession) {
          hotkeys.disable();
          router.replace({ type: "lock" });
        } else if (session.data.status === Session.Status.UnsealedSession) {
          hotkeys.enable();
          if (
            $activeRouteStore.type === "onboarding" ||
            $activeRouteStore.type === "lock" ||
            $activeRouteStore.type === "boot"
          ) {
            router.replace({ type: "profile" });
          }
        } else if (session.data.status === Session.Status.ProxyDown) {
          router.replace({ type: "onboarding" });
        } else {
          unreachable(session.data);
        }
        break;

      case remote.Status.Error:
        notification.showException(session.error);
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

<style>
  .settings {
    right: 132px;
    top: 24px;
    position: absolute;
    z-index: 10;
    display: flex;
    align-items: center;
    gap: 1rem;
  }
</style>

<UnrecoverableErrorScreen />
<Hotkeys />
<ModalLayout />
<Notifications />
<Theme />

{#if $sessionStore.status === remote.Status.Success}
  {#if $activeRouteStore.type === "designSystemGuide"}
    <DesignSystemShowcaseModal onClose={() => router.pop()}>
      <div class="settings" slot="top">
        <Tooltip value="Theme" position="bottom">
          <ThemeSetting />
        </Tooltip>
        <Tooltip value="UI font" position="bottom">
          <UiFontSetting />
        </Tooltip>
        <Tooltip value="Code font" position="bottom">
          <CodeFontSetting />
        </Tooltip>
        <Tooltip value="Primary color" position="bottom">
          <PrimaryColorSetting />
        </Tooltip>
      </div>

      <div slot="bottom">
        <SharedComponentShowcase />
      </div>
    </DesignSystemShowcaseModal>
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
  {:else if $activeRouteStore.type === "diagnostics"}
    <DiagnosticsScreen activeTab={$activeRouteStore.activeTab} />
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
    <PushPullNetworkScreen />
  {:else if $activeRouteStore.type === "orgs"}
    <OrgsScreen />
  {:else if $activeRouteStore.type === "settings"}
    <SettingsScreen />
  {:else if $activeRouteStore.type === "wallet"}
    <WalletScreen activeTab={$activeRouteStore.activeTab} />
  {:else if $activeRouteStore.type === "boot"}
    <Loading style="width: 100vw; height: 100vh;" />
  {:else}
    {unreachable($activeRouteStore)}
  {/if}
{:else}
  <Loading style="width: 100vw; height: 100vh;" />
{/if}
