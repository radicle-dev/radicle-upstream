<script>
  import DoubleSidebarLayout from "../layouts/DoubleSidebarLayout.svelte";
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import * as path from "../path.js";

  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";
  import Layout from "../layouts/SidebarHeaderLayout.svelte";
  import ProjectSidebar from "../components/ProjectSidebar.svelte";
  import ProjectHeader from "../components/ProjectHeader.svelte";

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

  setContext("projectId", params.id);

  const routes = {
    "/projects/:id/": Overview,
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
  .container {
    position: fixed;
  }

  .layout {
    margin-top: 40px;
  }
</style>

<Layout dataCy="page">
  {#await $project then result}
    <ProjectHeader
      style="position: fixed; top: 0;"
      project={result.data.project} />
    <NotificationFaucet />
    <div class="container">
      <div class="layout">
        <Router {routes} />
      </div>
    </div>
  {/await}
</Layout>
