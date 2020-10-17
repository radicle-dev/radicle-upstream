<script>
  import { getContext } from "svelte";

  import { isExperimental, openPath } from "../../native/ipc.js";
  import Router from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import * as path from "../src/path.ts";
  import {
    checkout,
    fetch,
    isMaintainer,
    project as store,
  } from "../src/project.ts";
  import * as screen from "../src/screen.ts";
  import {
    commits as commitsStore,
    currentRevision,
    currentPeerId,
    fetchCommits,
    fetchRevisions,
    resetCurrentRevision,
    resetCurrentPeerId,
    revisions as revisionsStore,
  } from "../src/source.ts";
  import { CSSPosition } from "../src/style.ts";

  import {
    Header,
    HorizontalMenu,
    Remote,
    RevisionSelector,
    SidebarLayout,
    Tooltip,
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
    "/projects/:id/commits": Commits,
    "/projects/:id/revisions": Revisions,
  };

  export let params = null;
  const projectId = params.id;
  const session = getContext("session");
  const trackTooltipMaintainer = "You can't unfollow your own project";
  const trackTooltip = "Unfollowing is not yet supported";

  // Reset some stores on first load
  resetCurrentRevision();
  resetCurrentPeerId();

  $: topbarMenuItems = (project, commitCounter) => {
    const items = [
      {
        icon: Icon.House,
        title: "Source",
        href: path.projectSource(project.id),
        looseActiveStateMatching: true,
      },
      {
        icon: Icon.Commit,
        title: "Commits",
        counter: commitCounter,
        href: path.projectCommits(project.id),
        looseActiveStateMatching: true,
      },
    ];
    isExperimental() &&
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

  const handleCheckout = async (event, project, peerId) => {
    try {
      screen.lock();
      const path = await checkout(
        project.id,
        event.detail.checkoutDirectoryPath,
        peerId
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
  fetchRevisions({ projectId });
  $: if ($currentRevision)
    fetchCommits({ projectId, revision: $currentRevision });
</script>

<style>
  .revision-selector-wrapper {
    width: 18rem;
    position: relative;
    margin-right: 2rem;
  }
</style>

<SidebarLayout dataCy="project-screen">
  <Remote {store} let:data={project} context="project">
    <Remote store={revisionsStore} let:data={revisions} context="revisions">
      <Header.Large
        name={project.metadata.name}
        urn={project.id}
        description={project.metadata.description}
        stats={project.stats}>
        <div slot="left">
          <div style="display: flex">
            <div class="revision-selector-wrapper">
              {#if $currentPeerId}
                <RevisionSelector
                  currentPeerId={$currentPeerId}
                  bind:currentRevision={$currentRevision}
                  {revisions} />
              {/if}
            </div>
            <Remote store={commitsStore} let:data={commits}>
              <HorizontalMenu
                items={topbarMenuItems(project, commits.stats.commits)} />
              <div slot="loading">
                <HorizontalMenu
                  items={topbarMenuItems(project, null)}
                  style="display: inline" />
              </div>
            </Remote>
          </div>
        </div>
        <div slot="right">
          <CheckoutButton
            on:checkout={ev => handleCheckout(ev, project, $currentPeerId)}
            projectName={project.metadata.name} />
        </div>
        <div slot="top">
          <div style="display: flex">
            <PeerSelector
              bind:currentPeerId={$currentPeerId}
              {revisions}
              on:select={() => {
                resetCurrentRevision();
              }} />
            <Tooltip
              position={CSSPosition.Left}
              value={isMaintainer(project, session.identity.urn) ? trackTooltipMaintainer : trackTooltip}>
              <TrackToggle blocked expanded tracking />
            </Tooltip>
          </div>
        </div>
      </Header.Large>
    </Remote>
    <Router {routes} />
  </Remote>
</SidebarLayout>
