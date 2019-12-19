<script>
  import DoubleSidebarLayout from "../layouts/DoubleSidebarLayout.svelte";
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import * as path from "../path.js";

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

  import NotificationFaucet from "../components/NotificationFaucet.svelte";

  export let params = null;

  const id = { domain: params.domain, name: params.name };
  setContext("projectId", id);

  const routes = {
    "/projects/:domain/:name/": Overview,
    "/projects/:domain/:name/overview": Overview,
    "/projects/:domain/:name/feed": Feed,
    "/projects/:domain/:name/members": Members,
    "/projects/:domain/:name/funds": Funds,
    "/projects/:domain/:name/source": Source,
    "/projects/:domain/:name/source/*": Source,
    "/projects/:domain/:name/commits": Commits,
    "/projects/:domain/:name/branches": Branches,
    "*": NotFound
  };

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: IdInput!) {
      project(id: $id) {
        id {
          domain
          name
        }
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
    variables: { id }
  });

  $: revision.set(path.extractProjectSourceRevision($location));
  $: objectPath.set(path.extractProjectSourceObjectPath($location));
  $: objectType.set(path.extractProjectSourceObjectType($location));
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

<DoubleSidebarLayout>
  {#await $project then result}
    <ProjectSidebar />

    <div class="container">
      <NotificationFaucet />
      <ProjectBreadcrumbs
        style="position: fixed; top: 0;"
        project={result.data.project} />
      <div class="layout">
        <Router {routes} />
      </div>
    </div>
  {/await}
</DoubleSidebarLayout>
