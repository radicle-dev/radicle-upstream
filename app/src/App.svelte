<script>
  import ApolloClient from "apollo-boost";
  import {
    InMemoryCache,
    defaultDataIdFromObject
  } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { push, pop, location } from "svelte-spa-router";
  import * as path from "./lib/path.js";
  import { hash } from "./lib/hash.js";

  import CreateProject from "./Screens/CreateProject.svelte";
  import DesignSystemGuide from "./Screens/DesignSystemGuide.svelte";
  import Feed from "./Screens/Feed.svelte";
  import Help from "./Screens/Help.svelte";
  import NotFound from "./Screens/NotFound.svelte";
  import Profile from "./Screens/Profile.svelte";
  import Project from "./Screens/Project.svelte";
  import Projects from "./Screens/Projects.svelte";
  import RegisterProject from "./Screens/RegisterProject.svelte";
  import Search from "./Screens/Search.svelte";
  import Wallet from "./Screens/Wallet.svelte";

  import hotkeys from "hotkeys-js";

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
    "/search": Search,
    "/feed": Feed,
    "/projects": Projects,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/wallet": Wallet,
    "/profile": Profile,
    "/help": Help,
    "*": NotFound
  };

  hotkeys("shift+d", () => {
    if (path.active(path.designSystemGuide(), $location)) {
      pop();
    }
    push(path.designSystemGuide());
  });

  hotkeys("shift+/", () => {
    if (path.active(path.help(), $location)) {
      pop();
    }
    push(path.help());
  });

  hotkeys("esc", () => {
    if (
      path.active(path.help(), $location) ||
      path.active(path.designSystemGuide(), $location)
    ) {
      pop();
    }
  });
</script>

<Router {routes} />
