<script>
  import { setContext } from "svelte";
  import { revision, objectPath, objectType } from "../stores.js";
  import { DEFAULT_PROJECT_REVISION } from "../config.js";
  import { BLOB, TREE } from "../types.js";

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

  const PATH_MATCH = new RegExp(`/source/(.*)/(${BLOB}|${TREE})(.*)`);

  $: rev = $location.match(PATH_MATCH);
  $: rev = rev === null ? DEFAULT_PROJECT_REVISION : rev[1];
  $: revision.set(rev);

  $: path = $location.match(PATH_MATCH);
  $: path = path === null ? "/" : path[3];
  $: objectPath.set(path);

  $: type = $location.match(PATH_MATCH);
  $: type = type === null ? TREE : type[2];
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
