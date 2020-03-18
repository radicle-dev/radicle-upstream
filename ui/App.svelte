<script>
  import ApolloClient from "apollo-boost";
  import {
    InMemoryCache,
    defaultDataIdFromObject
  } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { hash } from "./lib/hash.js";
  import { initializeHotkeys } from "./lib/hotkeys.js";

  import CreateProject from "./Screen/CreateProject.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Help from "./Screen/Help.svelte";
  import Network from "./Screen/Network.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Project from "./Screen/Project.svelte";
  import Projects from "./Screen/Projects.svelte";
  import RegisterProject from "./Screen/RegisterProject.svelte";
  import Search from "./Screen/Search.svelte";
  import CreateIdentity from "./Screen/IdentityCreation.svelte";

  initializeHotkeys();

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql",
    cache: new InMemoryCache({
      dataIdFromObject: object => {
        switch (object.__typename) {
          case "Project":
            return hash(JSON.stringify(object));
          default:
            return defaultDataIdFromObject(object);
        }
      }
    })
  });

  setClient(client);

  const routes = {
    "/identity/new": CreateIdentity,
    "/search": Search,
    "/network": Network,
    "/projects": Projects,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/help": Help,
    "*": NotFound
  };
</script>

<Router {routes} />
