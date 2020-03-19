<script>
  import { setContext } from "svelte";
  import {
    revisionStore,
    objectPathStore,
    objectTypeStore
  } from "../store/sourceBrowser.js";
  import * as path from "../lib/path.js";

  import Router, { location, push } from "svelte-spa-router";
  import { SidebarLayout, Topbar } from "../DesignSystem/Component";
  import { Button, Icon } from "../DesignSystem/Primitive";

  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import NotFound from "./NotFound.svelte";

  export let params = null;

  setContext("projectId", params.id);

  const routes = {
    "/projects/:id/": Source,
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

  $: revisionStore.set(path.extractProjectSourceRevision($location));
  $: objectPathStore.set(path.extractProjectSourceObjectPath($location));
  $: objectTypeStore.set(path.extractProjectSourceObjectType($location));

  const topbarMenuItems = projectId => [
    {
      icon: Icon.Home,
      title: "Source",
      href: path.projectSource(projectId),
      looseActiveStateMatching: true
    },
    {
      icon: Icon.Issue,
      title: "Issues",
      href: path.projectIssues(projectId),
      looseActiveStateMatching: false
    },
    {
      icon: Icon.Revisions,
      title: "Revisions",
      href: path.projectRevisions(projectId),
      looseActiveStateMatching: false
    }
  ];
</script>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  {#await $project then result}
    <Topbar
      style="position: fixed; top: 0;"
      avatarUrl={result.data.project.metadata.imgUrl}
      name={result.data.project.metadata.name}
      href={path.projectOverview(result.data.project.id)}
      menuItems={topbarMenuItems(result.data.project.id)}>
      <Button
        variant="secondary"
        size="small"
        style="margin-right: 16px"
        on:click={() => {
          push(path.registerProject(result.data.project.id));
        }}>
        <span>Register</span>
      </Button>
    </Topbar>
    <Router {routes} />
  {/await}
</SidebarLayout>
