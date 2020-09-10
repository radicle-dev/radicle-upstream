<script>
  import { isDev, openPath } from "../../native/ipc.js";
  import Router, { push } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import * as path from "../src/path.ts";
  import { checkout, fetch, project as store } from "../src/project.ts";
  import * as screen from "../src/screen.ts";
  import { revisions as revisionsStore } from "../src/source.ts";

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
  import CheckoutButton from "./Project/CheckoutButton.svelte";
  import PeerSelector from "./Project/PeerSelector.svelte";

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
  let currentPeerId;

  $: topbarMenuItems = projectId => {
    const items = [
      {
        icon: Icon.House,
        title: "Source",
        href: path.projectSource(projectId, currentPeerId),
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

  const handleCheckout = async (event, project) => {
    try {
      screen.lock();
      const path = await checkout(
        project.id,
        event.detail.checkoutDirectoryPath,
        "PEER_ID_GOES_HERE",
        "BRANCH_TO_CHECK_OUT_GOES_HERE"
      );

      notification.info(
        `${project.metadata.name} checked out to ${path}`,
        true,
        "Open folder",
        () => {
          openPath(path);
        }
      );
    } catch (error) {
      notification.error(`Checkout failed: ${error.message}`, true);
    } finally {
      screen.unlock();
    }
  };

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
      <div slot="right">
        <CheckoutButton
          on:checkout={ev => handleCheckout(ev, project)}
          projectName={project.metadata.name} />
      </div>
      <div slot="top">
        <div style="display: flex">
          <Remote store={revisionsStore} let:data={revisions}>
            <PeerSelector
              {currentPeerId}
              maintainers={project.maintainers}
              {revisions}
              on:select={event => {
                currentPeerId = event.detail.peerId;
                push(path.projectSource(projectId, currentPeerId));
              }} />
          </Remote>
          <TrackToggle />
        </div>
      </div>
    </Header.Large>
    <Router {routes} />
  </Remote>
</SidebarLayout>
