<script>
  import { isDev } from "../../native/ipc.js";
  import Router, { link } from "svelte-spa-router";

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
  import Issue from "./Project/Issue.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import Commit from "./Project/Commit.svelte";
  import Commits from "./Project/Commits.svelte";

  import SourceMenu from "./Project/SourceMenu.svelte";
  import IssuesMenu from "./Project/IssuesMenu.svelte";
  import RevisionsMenu from "./Project/RevisionsMenu.svelte";

  const routes = {
    "/projects/:id/": Source,
    "/projects/:id/source": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/issue": Issue,
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
  const projectId = params.id;
  let codeCollabMenuItems;
  const topbarMenuItems = projectId => {
    const items = [
      {
        icon: Icon.House,
        title: "Source",
        href: path.projectSource(projectId),
        looseActiveStateMatching: true,
      },
    ];
    isDev() &&
      items.push(
        {
          icon: Icon.ExclamationCircle,
          title: "Issues",
          href: path.projectIssues(projectId),
          looseActiveStateMatching: false,
        },
        {
          icon: Icon.Revision,
          title: "Revisions",
          href: path.projectRevisions(projectId),
          looseActiveStateMatching: false,
        }
      );
    return items;
  };

  if (isDev()) {
    codeCollabMenuItems = [
      {
        title: "New issue",
        icon: Icon.ExclamationCircle,
        event: () => console.log("event(new-issue)"),
      },
      {
        title: "New revision",
        icon: Icon.Revision,
        event: () => console.log("event(new-revision)"),
      },
    ];
  }

  fetch({ id: projectId });
</script>

<SidebarLayout
  style="margin-top: var(--topbar-height);"
  dataCy="project-screen">
  <Remote {store} let:data={project} context="project">
    <Topbar style="position: fixed; top: 0;">
      <a slot="left" href={path.projectSource(projectId)} use:link>
        <!-- TODO(rudolfs): show whether the project is registered under user or org -->
        <Breadcrumb
          title={project.metadata.name}
          user={project.registered}
          org={project.registered} />
      </a>

      <div slot="middle">
        <HorizontalMenu items={topbarMenuItems(project.id)} />
      </div>

      <div slot="right" style="display: flex">
        <Router routes={menuRoutes} />
        <TrackToggle style="margin-left: 16px" />
        <AdditionalActionsDropdown
          dataCy="context-menu"
          style="margin: 0 24px 0 16px"
          headerTitle={project.shareableEntityIdentifier}
          menuItems={codeCollabMenuItems} />
      </div>
    </Topbar>
    <Router {routes} />
  </Remote>
</SidebarLayout>
