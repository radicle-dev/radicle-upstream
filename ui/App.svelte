<script>
  import { push, location } from "svelte-spa-router";

  import * as navigation from "./src/navigation.ts";
  import * as notification from "./src/notification.ts";
  import * as path from "./src/path.ts";
  import * as remote from "./src/remote.ts";
  import { clear, fetch, session as store } from "./src/session.ts";

  import { NotificationFaucet, Remote } from "./DesignSystem/Component";
  import { Button, Title } from "./DesignSystem/Primitive";

  import Hotkeys from "./Hotkeys.svelte";
  import Navigation from "./Navigation.svelte";
  import Theme from "./Theme.svelte";

  import Blank from "./Screen/Blank.svelte";
  /* import IdentityCreation from "./Screen/IdentityCreation.svelte"; */
  /* import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte"; */
  import Help from "./Screen/Help.svelte";
  /* import NotFound from "./Screen/NotFound.svelte"; */
  /* import Org from "./Screen/Org.svelte"; */
  /* import OrgRegistration from "./Screen/OrgRegistration.svelte"; */
  /* import MemberRegistration from "./Screen/Org/MemberRegistration.svelte"; */
  /* import Profile from "./Screen/Profile.svelte"; */
  /* import Project from "./Screen/Project.svelte"; */
  /* import ProjectCreation from "./Screen/ProjectCreation.svelte"; */
  /* import ProjectRegistration from "./Screen/ProjectRegistration.svelte"; */
  /* import Search from "./Screen/Search.svelte"; */
  /* import Settings from "./Screen/Settings.svelte"; */
  /* import TransactionDetails from "./Screen/TransactionDetails.svelte"; */
  /* import UserRegistration from "./Screen/UserRegistration.svelte"; */

  /* const routes = { */
  /*   "/": Blank, */
  /*   "/identity/new": IdentityCreation, */
  /*   "/search": Search, */
  /*   "/settings": Settings, */
  /*   "/profile/*": Profile, */
  /*   "/orgs/register": OrgRegistration, */
  /*   "/orgs/:id/members/register": MemberRegistration, */
  /*   "/orgs/:id": Org, */
  /*   "/orgs/:id/*": Org, */
  /*   "/projects/new": ProjectCreation, */
  /*   "/projects/register/:domainId": ProjectRegistration, */
  /*   "/projects/:projectId/register/:domainId": ProjectRegistration, */
  /*   "/projects/:id/*": Project, */
  /*   "/design-system-guide": DesignSystemGuide, */
  /*   "/help": Help, */
  /*   "/user-registration": UserRegistration, */
  /*   "/transactions/:id": TransactionDetails, */
  /*   "*": NotFound, */
  /* }; */

  const views = {};
  views[navigation.Screen.Blank] = { component: Blank };
  views[navigation.Screen.Help] = { component: Help };

  console.log(views);

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
</style>

<Hotkeys />
<NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />
<Theme />
<Remote {store} context="session">
  <Navigation {views} />

  <div slot="error" class="error">
    <Title variant="big" style="margin-bottom: 32px;">
      We're having trouble logging you into radicle. 😪
    </Title>
    <Button on:click={clear}>Clear Session</Button>
  </div>
</Remote>
