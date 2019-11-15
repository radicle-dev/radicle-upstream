<script>
  import ApolloClient from "apollo-boost";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import Sidebar from "./components/Sidebar.svelte";

  import DesignSystem from "./pages/DesignSystem.svelte";
  import Feed from "./pages/Feed.svelte";
  import NotFound from "./pages/NotFound.svelte";
  import Profile from "./pages/Profile.svelte";
  import Projects from "./pages/Projects.svelte";
  import Project from "./pages/Project.svelte";
  import Search from "./pages/Search.svelte";
  import Wallet from "./pages/Wallet.svelte";

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql"
  });

  setClient(client);

  const routes = {
    "/search": Search,
    "/feed": Feed,
    "/projects": Projects,
    "/projects/:id/:subpage*": Project,
    "/design-system": DesignSystem,
    "/wallet": Wallet,
    "/profile": Profile,
    "*": NotFound
  };
</script>

<style>
  .container {
    position: relative;
    left: var(--slim-sidebar-width);
    width: calc(100vw - var(--slim-sidebar-width));
    overflow-x: hidden;
  }
</style>

<div>
  <Sidebar />
  <div class="container">
    <Router {routes} />
  </div>
</div>
