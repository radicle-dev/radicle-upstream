<script>
  import { getContext } from "svelte";
  import Router, { link, push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { fetch, project as store } from "../src/project.ts";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    Remote,
    SidebarLayout,
    Topbar,
    TrackToggle,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Breadcrumb from "./Project/Breadcrumb.svelte";
  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import Commit from "./Project/Commit.svelte";
  import Commits from "./Project/Commits.svelte";

  import SourceMenu from "./Project/SourceMenu.svelte";
  import IssuesMenu from "./Project/IssuesMenu.svelte";
  import RevisionsMenu from "./Project/RevisionsMenu.svelte";

  const routes = {
    "/projects/:id/": Source,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/commit/:hash": Commit,
    "/projects/:id/commits/:branch": Commits,
    "/projects/:id/revisions": Revisions,
  };

  const menuRoutes = {
    "/projects/:id/": SourceMenu,
    "/projects/:id/source": SourceMenu,
    "/projects/:id/source/*": SourceMenu,
    "/projects/:id/issues": IssuesMenu,
    "/projects/:id/revisions": RevisionsMenu,
  };

  export let params = null;

  const topbarMenuItems = (projectId) => [
    {
      icon: Icon.Home,
      title: "Source",
      href: path.projectSource(projectId),
      looseActiveStateMatching: true,
    },
    {
      icon: Icon.Issue,
      title: "Issues",
      href: path.projectIssues(projectId),
      looseActiveStateMatching: false,
    },
    {
      icon: Icon.Revisions,
      title: "Revisions",
      href: path.projectRevisions(projectId),
      looseActiveStateMatching: false,
    },
  ];

  $: dropdownMenuItems = [
    registerProjectMenuItem,
    {
      title: "New issue",
      icon: Icon.Issue,
      event: () => console.log("event(new-issue)"),
    },
    {
      title: "New revision",
      icon: Icon.Revisions,
      event: () => console.log("event(new-revision)"),
    },
  ];

  const session = getContext("session");

  let registerProjectMenuItem;

  if (session.identity.registered) {
    registerProjectMenuItem = {
      dataCy: "register-project",
      title: "Register project",
      icon: Icon.Register,
      event: () =>
        push(path.registerExistingProject(params.id, session.identity.id)),
    };
  } else {
    registerProjectMenuItem = {
      dataCy: "register-project",
      title: "Register project",
      icon: Icon.Register,
      disabled: true,
      tooltip:
        "To unlock project registration, register your own handle first.",
    };
  }

  fetch({ id: params.id });
</script>

<SidebarLayout
  style="margin: calc(var(--topbar-height)) 0 0 0"
  dataCy="project-screen">
  <Remote {store} let:data={project} context="project">
    <Topbar style="position: fixed; top: 0;">
      <a slot="left" href={path.projectSource(params.id)} use:link>
        <!-- TODO(rudolfs): show whether the project is registered under user or org -->
        <Breadcrumb
          title={project.metadata.name}
          user={project.registered}
          org={project.registered} />
      </a>

      <div slot="middle">
        <HorizontalMenu items={topbarMenuItems(params.id)} />
      </div>

      <div slot="right" style="display: flex">
        <Router routes={menuRoutes} />
        <TrackToggle style="margin-left: 16px" peerCount="1.3k" />
        <AdditionalActionsDropdown
          dataCy="context-menu"
          style="margin: 0 24px 0 16px"
          headerTitle={project.shareableEntityIdentifier}
          menuItems={dropdownMenuItems} />
      </div>
    </Topbar>
    <Router {routes} />
  </Remote>
</SidebarLayout>
