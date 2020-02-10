<script>
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import * as path from "../path.js";

  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";
  import ProjectTopbar from "../components/ProjectTopbar.svelte";
  import SidebarLayout from "../layouts/SidebarLayout.svelte";

  import Overview from "./Project/Overview.svelte";
  import Feed from "./Project/Feed.svelte";
  import Funds from "./Project/Funds.svelte";
  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import NotFound from "./NotFound.svelte";

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

<SidebarLayout style="margin-top: calc(var(--topbar-height) + 33px)">
  {#await $project then result}
    <ProjectTopbar
      style="position: fixed; top: 0;"
      avatarUrl={result.data.project.metadata.imgUrl}
      id={result.data.project.id}
      name={result.data.project.metadata.name} />

    <Router {routes} />
  {/await}
</SidebarLayout>
