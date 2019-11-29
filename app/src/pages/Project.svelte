<script>
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import { DEFAULT_PROJECT_REVISION } from "../config.js";

  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";
  import ProjectSidebar from "../components/ProjectSidebar.svelte";
  import ProjectBreadcrumbs from "../components/ProjectBreadcrumbs.svelte";

  import Overview from "./Project/Overview.svelte";
  import Feed from "./Project/Feed.svelte";
  import Members from "./Project/Members.svelte";
  import Funds from "./Project/Funds.svelte";
  import Source from "./Project/Source.svelte";
  import Commits from "./Project/Commits.svelte";
  import Branches from "./Project/Branches.svelte";
  import NotFound from "./NotFound.svelte";

  export let params = null;

  setContext("projectId", params.id);

  const routes = {
    "/projects/:id/overview": Overview,
    "/projects/:id/feed": Feed,
    "/projects/:id/members": Members,
    "/projects/:id/funds": Funds,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/commits": Commits,
    "/projects/:id/branches": Branches,
    "*": NotFound
  };

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: ProjectId!) {
      project(id: $id) {
        id
        name
        description
        imgUrl
        members {
          keyName
          avatarUrl
        }
      }
    }
  `;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { id: params.id }
  });

  const PATH_MATCH = /\/source\/(.*)\/(blob|tree)(.*)/;

  $: rev = $location.match(PATH_MATCH);
  $: rev = rev === null ? DEFAULT_PROJECT_REVISION : rev[1];
  $: revision.set(rev);

  $: path = $location.match(PATH_MATCH);
  $: path = path === null ? "/" : path[3];
  $: objectPath.set(path);

  $: type = $location.match(PATH_MATCH);
  $: type = type === null ? "tree" : type[2];
  $: objectType.set(type);
</script>

<style>
  .container {
    position: relative;
    left: calc(var(--project-sidebar-width));
    width: calc(100vw - var(--sidebar-total-width));
  }

  .layout {
    margin: 96px 81px 64px 81px;
  }
</style>

{#await $project then result}
  <ProjectSidebar />

  <div class="container">
    <ProjectBreadcrumbs
      style="position: fixed; top: 0;"
      project={result.data.project} />
    <div class="layout">
      <Router {routes} />
    </div>
  </div>
{/await}
