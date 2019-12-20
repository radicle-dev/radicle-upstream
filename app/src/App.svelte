<script>
  import ApolloClient from "apollo-boost";
  import {
    InMemoryCache,
    defaultDataIdFromObject
  } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { push, pop, location } from "svelte-spa-router";
  import * as path from "./path.js";

  import DesignSystem from "./pages/DesignSystem.svelte";
  import Feed from "./pages/Feed.svelte";
  import NotFound from "./pages/NotFound.svelte";
  import Profile from "./pages/Profile.svelte";
  import Projects from "./pages/Projects.svelte";
  import CreateProject from "./pages/CreateProject.svelte";
  import Project from "./pages/Project.svelte";
  import Search from "./pages/Search.svelte";
  import Wallet from "./pages/Wallet.svelte";
  import Help from "./pages/Help.svelte";

  import hotkeys from "hotkeys-js";

  const hash = s =>
    s.split("").reduce((a, b) => ((a << 5) - a + b.charCodeAt(0)) | 0, 0);

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
    "/projects/:id/*": Project,
    "/design-system": DesignSystem,
    "/wallet": Wallet,
    "/profile": Profile,
    "/help": Help,
    "*": NotFound
  };

  hotkeys("d,shift+d", () => {
    if (path.active(path.designSystem(), $location)) {
      pop();
    }
    push(path.designSystem());
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
      path.active(path.designSystem(), $location)
    ) {
      pop();
    }
  });
</script>

<Router {routes} />
