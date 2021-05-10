<script>
  import { push, state, Router } from "ui/src/router.ts";

  import * as hotkeys from "./src/hotkeys.ts";
  import { isExperimental } from "./src/config";
  import "./src/localPeer.ts";
  import * as remote from "./src/remote.ts";
  import * as error from "./src/error.ts";
  import * as customProtocolHandler from "./src/customProtocolHandler.ts";
  import { fetch, session as store, Status } from "./src/session.ts";

  import {
    EmptyState,
    NotificationFaucet,
    Remote,
    ModalOverlay,
  } from "./DesignSystem/Component";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";

  import TransactionCenter from "./App/TransactionCenter.svelte";

  import BsodScreen from "ui/Screen/Bsod.svelte";
  import OnboardingScreen from "ui/Screen/Onboarding.svelte";
  import LockScreen from "ui/Screen/Lock.svelte";
  import DesignSystemGuideScreen from "ui/Screen/DesignSystemGuide.svelte";
  import ProfileScreen from "ui/Screen/Profile.svelte";

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.status === Status.NoSession) {
        hotkeys.disable();
        push({ component: OnboardingScreen });
      } else if ($store.data.status === Status.UnsealedSession) {
        hotkeys.enable();
        if (
          $state.component === null ||
          $state.component === OnboardingScreen ||
          $state.component === LockScreen
        ) {
          push({ component: ProfileScreen });
        }
      } else {
        hotkeys.disable();
        push({ component: LockScreen });
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

<BsodScreen />
<Hotkeys />
<ModalOverlay />
<NotificationFaucet />
<Theme />

{#if isExperimental && sessionIsUnsealed && $state.component !== DesignSystemGuideScreen}
  <TransactionCenter />
{/if}

<Remote {store} context="session" disableErrorLogging={true}>
  <Router />

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
