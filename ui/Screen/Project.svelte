<script>
  import { link } from "svelte-spa-router";
  import { setContext } from "svelte";
  import {
    revisionStore,
    objectPathStore,
    objectTypeStore
  } from "../store/sourceBrowser.js";
  import * as path from "../lib/path.js";

  import Router, { location, push } from "svelte-spa-router";
  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar,
    TrackToggle,
    IdentityAvatar
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";

  const routes = {
    "/projects/:id/": Source,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/revisions": Revisions
  };

  import SourceMenu from "./Project/SourceMenu.svelte";
  import IssuesMenu from "./Project/IssuesMenu.svelte";
  import RevisionsMenu from "./Project/RevisionsMenu.svelte";

  export let params = null;

  setContext("projectId", params.id);

  const menuRoutes = {
    "/projects/:id/": SourceMenu,
    "/projects/:id/source": SourceMenu,
    "/projects/:id/source/*": SourceMenu,
    "/projects/:id/issues": IssuesMenu,
    "/projects/:id/revisions": RevisionsMenu
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

  const dropdownMenuItems = [
    {
      title: "Register project",
      icon: Icon.Register,
      event: () => push(path.registerProject(params.id))
    },
    {
      title: "New issue",
      icon: Icon.Issue,
      event: () => console.log("event(new-issue)")
    },
    {
      title: "New revision",
      icon: Icon.Revisions,
      event: () => console.log("event(new-revision)")
    }
  ];
</script>

<style>
  .name {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-lightgray);
    padding-left: 16px;
    padding-right: 24px;
  }

  .right {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: flex-end;
  }
</style>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  {#await $project then result}
    <Topbar style="position: fixed; top: 0;">
      <a slot="left" class="name" href={path.profileProjects()} use:link>
        <IdentityAvatar
          showTitle={true}
          size={'regular'}
          style="color: var(--color-purple)" />
      </a>

      <div slot="middle">
        <HorizontalMenu items={topbarMenuItems()} />
      </div>

      <div slot="right" class="right">
        <Router routes={menuRoutes} />
        <TrackToggle style="margin-left: 16px" peerCount="1.3k" />
        <AdditionalActionsDropdown
          style="margin: 0 24px 0 16px"
          headerTitle={params.id}
          menuItems={dropdownMenuItems} />
      </div>
    </Topbar>
    <Router {routes} />
  {/await}
</SidebarLayout>
