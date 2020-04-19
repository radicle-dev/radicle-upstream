<script>
  import ApolloClient from "apollo-boost";
  import { InMemoryCache } from "apollo-cache-inmemory";
  import Router, { push } from "svelte-spa-router";
  import { setClient } from "svelte-apollo";

  import { initializeHotkeys } from "./lib/hotkeys.js";
  import * as path from "./lib/path.js";
  import { showNotification } from "./store/notification.js";
  import * as remote from "./src/remote.ts";
  import { fetch, session } from "./src/session.ts";

  import CreateProject from "./Screen/CreateProject.svelte";
  import Blank from "./Screen/Blank.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Help from "./Screen/Help.svelte";
  import Network from "./Screen/Network.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Org from "./Screen/Org.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import RegisterProject from "./Screen/RegisterProject.svelte";
  import UserRegistration from "./Screen/UserRegistration.svelte";
  import Search from "./Screen/Search.svelte";
  import TransactionDetails from "./Screen/TransactionDetails.svelte";
  import CreateIdentity from "./Screen/IdentityCreation.svelte";

  initializeHotkeys();

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql",
    cache: new InMemoryCache()
  });

  setClient(client);

  $: switch ($session.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($session.data.identity === null) {
        push(path.createIdentity());
      } else {
        push(path.profile());
      }
      break;

    case remote.Status.Error:
      console.log($session.error);
      showNotification({
        text: "Session could not be fetched",
        level: "error"
      });
      break;
  }

  const routes = {
    "/": Blank,
    "/identity/new": CreateIdentity,
    "/search": Search,
    "/network": Network,
    "/profile": Profile,
    "/profile/*": Profile,
    "/orgs/:id": Org,
    "/orgs/:id/*": Org,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/help": Help,
    "/user-registration": UserRegistration,
    "/transactions/:id": TransactionDetails,
    "*": NotFound
  };
</script>

<Router {routes} />
