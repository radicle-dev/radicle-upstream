<script>
  import Router, { push, location } from "svelte-spa-router";

  import * as notification from "./src/notification.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import { fetch, session as store } from "./src/session.ts";

  import { NotificationFaucet, Remote } from "./DesignSystem/Component";

  import Layout from "./Layout/Sidebar.svelte";

  import Hotkeys from "./App/Hotkeys.svelte";
  import Modal from "./App/Modal.svelte";
  import Theme from "./App/Theme.svelte";
  import TransactionCenter from "./App/TransactionCenter.svelte";

  import Blank from "./Screen/Blank.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Org from "./Screen/Org.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Settings from "./Screen/Settings.svelte";

  const routes = {
    "/": Blank,
    "/design-system-guide": DesignSystemGuide,
    "/orgs/:id": Org,
    "/orgs/:id/*": Org,
    "/profile/*": Profile,
    "/projects/:id/*": Project,
    "/settings": Settings,
    "*": NotFound,
  };

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.identity === null) {
        push(path.createIdentity());
      } else {
        if ($location === "/" || $location === "/identity/new") {
          push(path.profileProjects());
        }
      }
      break;

    case remote.Status.Error:
      console.error($store.error);
      notification.error("Session could not be fetched");
      break;
  }
  $: console.log($location);
</script>

<Hotkeys />
<NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />
<Theme />
<Remote {store} let:data={session} context="session">
  <Layout {session}>
    <Router {routes} />
  </Layout>
  <Modal />
  <TransactionCenter />
</Remote>
