<script>
  import Router, { push, location } from "svelte-spa-router";

  import * as modal from "./src/modal.ts";
  import * as notification from "./src/notification.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import { clear, fetch, session as store } from "./src/session.ts";

  import {
    NotificationFaucet,
    Remote,
    ModalOverlay,
  } from "./DesignSystem/Component";
  import { Button } from "./DesignSystem/Primitive";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";

  import Blank from "./Screen/Blank.svelte";
  import Onboarding from "./Screen/Onboarding.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Discovery from "./Screen/Discovery.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Org from "./Screen/Org.svelte";
  import OrgRegistration from "./Screen/OrgRegistration.svelte";
  import MemberRegistration from "./Screen/Org/MemberRegistration.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import ProjectCreation from "./Screen/ProjectCreation.svelte";
  import ProjectRegistration from "./Screen/ProjectRegistration.svelte";
  import SendFunds from "./Screen/SendFunds.svelte";
  import Settings from "./Screen/Settings.svelte";
  import TransactionDetails from "./Screen/TransactionDetails.svelte";
  import Untracked from "./Screen/Project/Untracked.svelte";
  import UserRegistration from "./Screen/UserRegistration.svelte";
  import UserProfile from "./Screen/UserProfile.svelte";

  const routes = {
    "/": Blank,
    "/onboarding": Onboarding,
    "/settings": Settings,
    "/discovery": Discovery,
    "/profile/*": Profile,
    "/orgs/register": OrgRegistration,
    "/orgs/:id/members/register": MemberRegistration,
    "/orgs/:id": Org,
    "/orgs/:id/*": Org,
    "/projects/untracked/:urn": Untracked,
    "/projects/new": ProjectCreation,
    "/projects/register/:domainId": ProjectRegistration,
    "/projects/:projectId/register/:domainId": ProjectRegistration,
    "/projects/:id/*": Project,
    "/user/:urn": UserProfile,
    "/user/:urn/*": UserProfile,
    "/design-system-guide": DesignSystemGuide,
    "/user-registration": UserRegistration,
    "/transactions/:id": TransactionDetails,
    "/send-funds": SendFunds,
    "*": NotFound,
  };

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.identity === null) {
        push(path.onboarding());
      } else {
        if ($location === path.blank() || $location === path.onboarding()) {
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
<ModalOverlay store={modal.store} />
<NotificationFaucet />
<Theme />

<Remote {store} context="session">
  <Router {routes} />

  <div slot="error" class="error">
    <h2>We're having trouble logging you into radicle. 😪</h2>
    <Button on:click={clear}>Clear Session</Button>
  </div>
</Remote>
