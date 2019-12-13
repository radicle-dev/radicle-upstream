<script>
  import ApolloClient from "apollo-boost";
  import {
    InMemoryCache,
    defaultDataIdFromObject
  } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";

  import DesignSystem from "./pages/DesignSystem.svelte";
  import Feed from "./pages/Feed.svelte";
  import NotFound from "./pages/NotFound.svelte";
  import Profile from "./pages/Profile.svelte";
  import Projects from "./pages/Projects.svelte";
  import CreateProject from "./pages/CreateProject.svelte";
  import Project from "./pages/Project.svelte";
  import Search from "./pages/Search.svelte";
  import Wallet from "./pages/Wallet.svelte";

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
    "/projects/:domain/:name/*": Project,
    "/design-system": DesignSystem,
    "/wallet": Wallet,
    "/profile": Profile,
    "*": NotFound
  };
</script>

<Router {routes} />
