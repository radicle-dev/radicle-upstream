<script lang="typescript">
  import * as router from "ui/src/router";
  import * as config from "ui/src/config";
  import * as customProtocolHandler from "ui/src/customProtocolHandler";
  import * as error from "ui/src/error";
  import * as hotkeys from "ui/src/hotkeys";
  import * as remote from "ui/src/remote";
  import { fetch, session as store, Status } from "ui/src/session";
  import "ui/src/localPeer";

  import {
    EmptyState,
    NotificationFaucet,
    ModalOverlay,
    Remote,
  } from "ui/DesignSystem/Component";

  import Bsod from "ui/Screen/Bsod.svelte";
  import DesignSystemGuide from "ui/Screen/DesignSystemGuide.svelte";
  import Lock from "ui/Screen/Lock.svelte";
  import Onboarding from "ui/Screen/Onboarding.svelte";
  import Profile from "ui/Screen/Profile.svelte";
  import UserProfile from "ui/Screen/UserProfile.svelte";
  import Project from "ui/Screen/Project.svelte";
  import Settings from "ui/Screen/Settings.svelte";

  import Hotkeys from "ui/Hotkeys.svelte";
  import Theme from "ui/Theme.svelte";
  import TransactionCenter from "ui/App/TransactionCenter.svelte";

  const activeRouteStore = router.activeRouteStore;

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.status === Status.NoSession) {
        hotkeys.disable();
        router.push({ type: "onboarding" });
      } else if ($store.data.status === Status.UnsealedSession) {
        hotkeys.enable();
        if (
          $activeRouteStore.type === "onboarding" ||
          $activeRouteStore.type === "lock"
        ) {
          router.push({ type: "profile", activeTab: "projects" });
        }
      } else {
        hotkeys.disable();
        router.push({ type: "lock" });
      }
      break;

    case remote.Status.Error:
      error.show($store.error);
      break;
  }

  $: sessionIsUnsealed =
    $store.status === remote.Status.Success &&
    $store.data.status === Status.UnsealedSession;

  customProtocolHandler.register();
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

{#if config.isExperimental && sessionIsUnsealed && $activeRouteStore.type !== "designSystemGuide"}
  <TransactionCenter />
{/if}

<Remote {store} context="session" disableErrorLogging={true}>
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
  {:else if $activeRouteStore.type === "project"}
    <Project
      activeTab={$activeRouteStore.activeTab}
      urn={$activeRouteStore.urn}
      commitHash={$activeRouteStore.commitHash} />
  {:else if $activeRouteStore.type === "settings"}
    <Settings />
  {:else}
    {router.unreachable($activeRouteStore)}
  {/if}

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
