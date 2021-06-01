<script lang="typescript">
  import Router, { push, location } from "svelte-spa-router";
  import wrap from "svelte-spa-router/wrap";

  import * as hotkeys from "./src/hotkeys";
  import "./src/localPeer";
  import * as path from "./src/path";
  import * as remote from "./src/remote";
  import * as screen from "./src/screen";
  import * as error from "./src/error";
  import * as org from "./src/org";
  import * as wallet from "./src/wallet";
  import * as customProtocolHandler from "./src/customProtocolHandler";
  import { fetch, session as store, Status } from "./src/session";

  import {
    EmptyState,
    NotificationFaucet,
    Remote,
    ModalOverlay,
  } from "./DesignSystem/Component";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";

  import Blank from "./Screen/Blank.svelte";
  import Bsod from "./Screen/Bsod.svelte";
  import Onboarding from "./Screen/Onboarding.svelte";
  import Lock from "./Screen/Lock.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Org from "./Screen/Org.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Settings from "./Screen/Settings.svelte";
  import Wallet from "./Screen/Wallet.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";
  import NetworkDiagnostics from "./Screen/NetworkDiagnostics.svelte";

  const orgWrap = wrap({
    component: Org,
    conditions: [
      async detail => {
        const match = detail.location.match(/\/org\/(.{42})/);
        let orgAddress;
        if (match) {
          orgAddress = match[1];
        } else {
          throw new Error("Org address not provided");
        }
        try {
          screen.lock();

          await org.fetchOrg(orgAddress);
          await org.resolveProjectAnchors(orgAddress);

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
    "/wallet/*": Wallet,
    "/wallet": Wallet,
    "/profile/*": Profile,
    "/projects/:urn/*": Project,
    "/projects/:urn": Project,
    "/org/:address/*": orgWrap,
    "/org/:address": orgWrap,
    "/user/:urn": UserProfile,
    "/user/:urn/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
    "/network-diagnostics/*": NetworkDiagnostics,
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

<Remote {store} context="session" disableErrorLogging={true}>
  <Router {routes} />

  <div slot="loading" class="error">
    <EmptyState headerText="Loading..." emoji="🕵️" text="" />
  </div>
</Remote>
