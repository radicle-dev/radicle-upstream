<script>
  import { isDev } from "../../native/ipc.js";
  import Router from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { fetch, project as store } from "../src/project.ts";

  import {
    Header,
    HorizontalMenu,
    Remote,
    SidebarLayout,
    TrackToggle,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Source from "./Project/Source.svelte";
  import Issues from "./Project/Issues.svelte";
  import Issue from "./Project/Issue.svelte";
  import Revisions from "./Project/Revisions.svelte";
  import Commit from "./Project/Commit.svelte";
  import Commits from "./Project/Commits.svelte";

  const routes = {
    "/projects/:id/": Source,
    "/projects/:id/source": Source,
    "/projects/:id/issues": Issues,
    "/projects/:id/issue": Issue,
    "/projects/:id/commit/:hash": Commit,
    "/projects/:id/commits/:branch": Commits,
    "/projects/:id/revisions": Revisions,
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

<SidebarLayout dataCy="project-screen">
  <Remote {store} let:data={project} context="project">
    <Header.Large
      variant="project"
      entity={project}
      style="position: fixed; top: 0;">
      <div slot="left">
        <HorizontalMenu items={topbarMenuItems(project.id)} />
      </div>
      <div slot="top">
        <TrackToggle />
      </div>
    </Header.Large>
    <Router {routes} />
  </Remote>
</SidebarLayout>
