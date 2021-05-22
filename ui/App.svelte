<script>
  import { push, routeStore } from "ui/src/router.ts";

  import * as hotkeys from "./src/hotkeys.ts";
  import { isExperimental } from "./src/config";
  import "./src/localPeer.ts";
  import * as remote from "./src/remote.ts";
  import * as error from "./src/error.ts";
  import * as customProtocolHandler from "./src/customProtocolHandler.ts";
  import { fetch, session as store, Status } from "./src/session.ts";

  import DesignSystemGuide from "ui/Screen/DesignSystemGuide.svelte";
  import Lock from "ui/Screen/Lock.svelte";
  import Onboarding from "ui/Screen/Onboarding.svelte";
  import Profile from "ui/Screen/Profile.svelte";
  import Project from "ui/Screen/Project.svelte";
  import Settings from "ui/Screen/Settings.svelte";

  import {
    EmptyState,
    NotificationFaucet,
    Remote,
    ModalOverlay,
  } from "./DesignSystem/Component";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";

  import TransactionCenter from "./App/TransactionCenter.svelte";

  import Bsod from "ui/Screen/Bsod.svelte";

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.status === Status.NoSession) {
        hotkeys.disable();
        push({ type: "onboarding" });
      } else if ($store.data.status === Status.UnsealedSession) {
        hotkeys.enable();
        if (
          $routeStore.type === "empty" ||
          $routeStore.type === "onboarding" ||
          $routeStore.type === "lock"
        ) {
          push({ type: "profile", activeTab: "projects" });
        }
      } else {
        hotkeys.disable();
        push({ type: "lock" });
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

{#if isExperimental && sessionIsUnsealed && $routeStore.type !== "designSystemGuide"}
  <TransactionCenter />
{/if}

<Remote {store} context="session" disableErrorLogging={true}>
  {#if $routeStore.type === "designSystemGuide"}
    <DesignSystemGuide />
  {:else if $routeStore.type === "lock"}
    <Lock />
  {:else if $routeStore.type === "onboarding"}
    <Onboarding />
  {:else if $routeStore.type === "profile"}
    <Profile activeTab={$routeStore.activeTab} />
  {:else if $routeStore.type === "project"}
    <Project
      activeTab={$routeStore.activeTab}
      urn={$routeStore.urn}
      commitHash={$routeStore.commitHash} />
  {:else if $routeStore.type === "settings"}
    <Settings />
  {/if}

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
