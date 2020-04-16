<script>
  import { setContext } from "svelte";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import Router, { link, location, push } from "svelte-spa-router";

  import { Icon } from "../DesignSystem/Primitive";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar,
    TrackToggle
  } from "../DesignSystem/Component";

  import { updateSourcePath } from "../src/sourceBrowser.ts";

  import * as path from "../lib/path.js";

  import Breadcrumb from "./Project/Breadcrumb.svelte";

  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import Commit from "./Project/Commit.svelte";

  import SourceMenu from "./Project/SourceMenu.svelte";
  import IssuesMenu from "./Project/IssuesMenu.svelte";
  import RevisionsMenu from "./Project/RevisionsMenu.svelte";

  const routes = {
    "/projects/:id/": Source,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/commits/:hash": Commit,
    "/projects/:id/revisions": Revisions
  };

  const menuRoutes = {
    "/projects/:id/": SourceMenu,
    "/projects/:id/source": SourceMenu,
    "/projects/:id/source/*": SourceMenu,
    "/projects/:id/issues": IssuesMenu,
    "/projects/:id/revisions": RevisionsMenu
  };

  export let params = null;
  setContext("projectId", params.id);

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $id) {
        id
        metadata {
          defaultBranch
          description
          name
        }
        registered {
          ... on OrgRegistration {
            orgId
          }
          ... on UserRegistration {
            userId
          }
        }
        stats {
          branches
          commits
          contributors
        }
      }
    }
  `;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { id: params.id }
  });

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

  $: updateSourcePath({ newPath: $location });
</script>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  {#await $project then result}
    <Topbar style="position: fixed; top: 0;">
      <a slot="left" href={path.projectSource(params.id)} use:link>
        <!-- TODO(rudolfs): show whether the project is registered under user or org -->
        <Breadcrumb
          title={result.data.project.metadata.name}
          user={result.data.project.registered}
          org={result.data.project.registered} />
      </a>

      <div slot="middle">
        <HorizontalMenu items={topbarMenuItems(params.id)} />
      </div>

      <div slot="right" style="display: flex">
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
