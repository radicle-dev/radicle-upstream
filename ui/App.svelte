<script>
  import Router, { push, location } from "svelte-spa-router";

  import * as notification from "./src/notification.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import { clear, fetch, session as store } from "./src/session.ts";

  import { NotificationFaucet, Remote } from "./DesignSystem/Component";
  import { Button } from "./DesignSystem/Primitive";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";

  import Blank from "./Screen/Blank.svelte";
  import IdentityCreation from "./Screen/IdentityCreation.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Discovery from "./Screen/Discovery.svelte";
  import Shortcuts from "./Screen/Shortcuts.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import ProjectCreation from "./Screen/ProjectCreation.svelte";
  import Search from "./Screen/Search.svelte";
  import Settings from "./Screen/Settings.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";

  const routes = {
    "/": Blank,
    "/identity/new": IdentityCreation,
    "/search": Search,
    "/settings": Settings,
    "/discovery": Discovery,
    "/profile/*": Profile,
    "/projects/new": ProjectCreation,
    "/projects/:id/*": Project,
    "/user/:id": UserProfile,
    "/user/:id/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
    "/shortcuts": Shortcuts,
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

  h2 {
    margin-bottom: 2rem;
  }
</style>

<Hotkeys />
<NotificationFaucet />
<Theme />
<Remote {store} context="session">
  <Router {routes} />

  <div slot="error" class="error">
    <h2>We're having trouble logging you into radicle. 😪</h2>
    <Button on:click={clear}>Clear Session</Button>
  </div>
</Remote>
