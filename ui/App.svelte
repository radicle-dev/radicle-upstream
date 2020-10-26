<script>
  import Router, { push, location } from "svelte-spa-router";

  import * as notification from "./src/notification.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import * as hotkeys from "./src/hotkeys.ts";

  import { fetch, session as store } from "./src/session.ts";

  import {
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
  import Discovery from "./Screen/Discovery.svelte";
  import ModalNewProject from "./Modal/NewProject.svelte";
  import ModalSearch from "./Modal/Search.svelte";
  import ModalShortcuts from "./Modal/Shortcuts.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Settings from "./Screen/Settings.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";

  const routes = {
    "/": Blank,
    "/onboarding": Onboarding,
    "/lock": Lock,
    "/settings": Settings,
    "/discovery": Discovery,
    "/profile/*": Profile,
    "/projects/:id/*": Project,
    "/user/:urn": UserProfile,
    "/user/:urn/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
    "*": NotFound,
  };

  const modalRoutes = {
    "/new-project": ModalNewProject,
    "/search": ModalSearch,
    "/shortcuts": ModalShortcuts,
  };

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data === null) {
        hotkeys.disable();
        push(path.onboarding());
      } else if ($store.data.identity) {
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
      console.error($store.error);
      notification.error("Session could not be fetched");
      break;
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

<Hotkeys />
<ModalOverlay {modalRoutes} />
<NotificationFaucet />
<Theme />

<Remote {store} context="session">
  <Router {routes} />

  <div slot="error" class="error">
    <Bsod />
  </div>
</Remote>
