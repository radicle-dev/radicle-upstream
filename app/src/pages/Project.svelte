<script>
  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";
  import ProjectSidebar from "../components/ProjectSidebar.svelte";

  import Overview from "./Project/Overview.svelte";
  import Feed from "./Project/Feed.svelte";
  import Members from "./Project/Members.svelte";
  import Funds from "./Project/Funds.svelte";
  import Source from "./Project/Source.svelte";
  import Commits from "./Project/Commits.svelte";
  import Branches from "./Project/Branches.svelte";
  import NotFound from "./NotFound.svelte";

  export let params = null;

  const routes = {
    "/projects/:id/overview": Overview,
    "/projects/:id/feed": Feed,
    "/projects/:id/members": Members,
    "/projects/:id/funds": Funds,
    "/projects/:id/source": Source,
    "/projects/:id/source/:head/*": Source,
    "/projects/:id/commits": Commits,
    "/projects/:id/branches": Branches,
    "*": NotFound
  };
</script>

<style>
  .container {
    position: relative;
    left: calc(var(--project-sidebar-width));
    width: calc(100vw - var(--sidebar-total-width));
  }

  .layout {
    margin: 64px 8vw 64px 8vw;
  }
</style>

<ProjectSidebar projectId={params.id} />

<div class="container">
  <div class="layout">
    <Router {routes} />
  </div>
</div>
