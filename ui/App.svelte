<script>
  import Router, { push, location } from "svelte-spa-router";

  import * as hotkeys from "./src/hotkeys.ts";
  import { isExperimental } from "./src/config";
  import "./src/localPeer.ts";
  import * as path from "./src/path.ts";
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

  import Blank from "./Screen/Blank.svelte";
  import Bsod from "./Screen/Bsod.svelte";
  import Onboarding from "./Screen/Onboarding.svelte";
  import Lock from "./Screen/Lock.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import ModalManagePeers from "./Modal/ManagePeers.svelte";
  import ModalNewProject from "./Modal/NewProject.svelte";
  import ModalSearch from "./Modal/Search.svelte";
  import ModalShortcuts from "./Modal/Shortcuts.svelte";
  import ModalTransaction from "./Modal/Transaction.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Settings from "./Screen/Settings.svelte";
  import ModalLinkAddress from "./Modal/Funding/LinkAddress.svelte";
  import ModalPoolOnboarding from "./Modal/Funding/Onboarding.svelte";
  import ModalWalletQRCode from "./Modal/Wallet/QRCode.svelte";
  import ModalTopUp from "./Modal/Funding/Pool/TopUp.svelte";
  import ModalWithdraw from "./Modal/Funding/Pool/Withdraw.svelte";
  import ModalCollect from "./Modal/Funding/Pool/Collect.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";

  const routes = {
    "/": Blank,
    "/onboarding": Onboarding,
    "/lock": Lock,
    "/settings": Settings,
    "/profile/*": Profile,
    "/projects/:urn/*": Project,
    "/projects/:urn": Project,
    "/user/:urn": UserProfile,
    "/user/:urn/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
    "*": NotFound,
  };

  const modalRoutes = {
    "/manage-peers": ModalManagePeers,
    "/new-project": ModalNewProject,
    "/search": ModalSearch,
    "/shortcuts": ModalShortcuts,
    "/wallet/qrcode": ModalWalletQRCode,
    "/funding/link": ModalLinkAddress,
    "/funding/pool/onboarding": ModalPoolOnboarding,
    "/funding/pool/collect": ModalCollect,
    "/funding/pool/withdraw": ModalWithdraw,
    "/funding/pool/top-up": ModalTopUp,
    "/transaction": ModalTransaction,
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
<ModalOverlay {modalRoutes} />
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
