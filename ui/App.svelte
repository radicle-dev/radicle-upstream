<script>
  import Router, { push, location } from "svelte-spa-router";
  import wrap from "svelte-spa-router/wrap";

  import * as hotkeys from "./src/hotkeys.ts";
  import { isExperimental } from "./src/config";
  import "./src/localPeer.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import * as screen from "./src/screen.ts";
  import * as error from "./src/error.ts";
  import * as org from "./src/org.ts";
  import * as wallet from "./src/wallet.ts";
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

  import Blank from "./Screen/Blank.svelte";
  import Bsod from "./Screen/Bsod.svelte";
  import Onboarding from "./Screen/Onboarding.svelte";
  import Lock from "./Screen/Lock.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Org from "./Screen/Org.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Settings from "./Screen/Settings.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";

  const orgWrap = wrap({
    component: Org,
    conditions: [
      async detail => {
        try {
          screen.lock();

          // TODO(rudolfs): clean this up.
          const orgAddress = detail.location.match(/\/org\/(.{42})/)[1];
          await org.fetchOrg(orgAddress);
          await org.resolveProjects(orgAddress);

          return true;
        } catch (error) {
          console.log(error);
          return false;
        } finally {
          screen.unlock();
        }
      },
    ],
  });

  const routes = {
    "/": Blank,
    "/onboarding": Onboarding,
    "/lock": Lock,
    "/settings": Settings,
    "/profile/*": Profile,
    "/projects/:urn/*": Project,
    "/projects/:urn": Project,
    "/org/:address/*": orgWrap,
    "/org/:address": orgWrap,
    "/user/:urn": UserProfile,
    "/user/:urn/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
  };

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.status === Status.NoSession) {
        hotkeys.disable();
        push(path.onboarding());
      } else if ($store.data.status === Status.UnsealedSession) {
        hotkeys.enable();
        if (
          $location === path.blank() ||
          $location === path.onboarding() ||
          $location === path.lock()
        ) {
          push(path.profileProjects());
        }
      } else {
        hotkeys.disable();
        push(path.lock());
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

{#if isExperimental && sessionIsUnsealed && $location !== path.designSystemGuide()}
  <TransactionCenter />
{/if}

<Remote {store} context="session" disableErrorLogging={true}>
  <Router {routes} />

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
