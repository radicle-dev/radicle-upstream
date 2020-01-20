<script>
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import * as path from "../path.js";

  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";
  import Sidebar from "../components/Sidebar.svelte";
  import ProjectTopbar from "../components/ProjectTopbar.svelte";

  import Overview from "./Project/Overview.svelte";
  import Feed from "./Project/Feed.svelte";
  import Funds from "./Project/Funds.svelte";
  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import NotFound from "./NotFound.svelte";

  import NotificationFaucet from "../components/NotificationFaucet.svelte";

  export let params = null;

  setContext("projectId", params.id);

  const routes = {
    "/projects/:id/": Overview,
    "/projects/:id/overview": Overview,
    "/projects/:id/feed": Feed,
    "/projects/:id/funds": Funds,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/revisions": Revisions,
    "*": NotFound
  };

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $id) {
        id
        metadata {
          name
          description
          imgUrl
        }
      }
    }
  `;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { id: params.id }
  });

  $: revision.set(path.extractProjectSourceRevision($location));
  $: objectPath.set(path.extractProjectSourceObjectPath($location));
  $: objectType.set(path.extractProjectSourceObjectType($location));
</script>

<style>
  .main-container {
    width: 1180px;
  }

  .main-layout {
    margin-top: 40px;
    width: inherit;
  }

  .project-container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    height: 100%;
    overflow-y: scroll;
  }

  .content {
    position: absolute;
    margin: 64px 96px 64px 96px;
  }
</style>

<div data-cy="project-layout">
  <Sidebar />
  <div data-cy="page-container" class="project-container">
    <div class="content">
      {#await $project then result}
        <ProjectTopbar
          style="position: fixed; top: 0;"
          avatarUrl={result.data.project.metadata.imgUrl}
          id={result.data.project.id}
          name={result.data.project.metadata.name} />
        <NotificationFaucet />
        <div class="main-container">
          <div class="main-layout">
            <Router {routes} />
          </div>
        </div>
      {/await}
    </div>
  </div>
</div>
