<script lang="typescript">
  import Router from "svelte-spa-router";
  import { isExperimental, openPath } from "../../native/ipc.js";

  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import { checkout, fetch, project as store } from "../src/project";
  import type { Project } from "../src/project"; // Annoying but necessary type import
  import * as remote from "../src/remote";
  import * as screen from "../src/screen";
  import {
    commits as commitsStore,
    currentPeerId,
    currentRevision as currentRevisionStore,
    resetCurrentRevision,
    resetCurrentPeerId,
    revisions as revisionsStore,
  } from "../src/source";
  import * as source from "../src/source";

  import {
    Header,
    HorizontalMenu,
    Remote,
    RevisionSelector,
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
    "/projects/:id/commits": Commits,
    "/projects/:id/revisions": Revisions,
  };

  export let params: { id: string };
  const projectId = params.id;

  // Reset user-manipulated stores on first load
  resetCurrentRevision();
  resetCurrentPeerId();

  $: topbarMenuItems = (commitCounter?: number) => {
    const items = [
      {
        icon: Icon.House,
        title: "Source",
        href: path.projectSource(projectId),
        looseActiveStateMatching: true,
      },
      {
        icon: Icon.Commit,
        title: "Commits",
        counter: commitCounter,
        href: path.projectCommits(projectId),
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

  const handleCheckout = async (
    event: CustomEvent<{ checkoutDirectoryPath: string }>,
    project: Project
  ) => {
    try {
      screen.lock();
      const path = await checkout(
        project.id,
        event.detail.checkoutDirectoryPath,
        $currentPeerId
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

  // Workaround for stores: https://github.com/sveltejs/language-tools/issues/493
  // TODO(sos): should be removed if/when this is fixed.
  let revisions: source.PeerRevisions[] | undefined;
  let project: Project | undefined;
  let commits: source.CommitsStore | undefined;
  let currentRevision: source.SupportedRevision | undefined;

  $: {
    const rs = $revisionsStore;
    revisions = rs.status === remote.Status.Success ? rs.data : undefined;
  }

  $: {
    const ps = $store;
    project = ps.status === remote.Status.Success ? ps.data : undefined;
  }

  $: {
    const cs = $commitsStore;
    commits = cs.status === remote.Status.Success ? cs.data : undefined;
  }

  const updatePeer = (ev: CustomEvent<{ peerId: string }>) => {
    source.updateCurrentPeerId({ peerId: ev.detail.peerId });
  };

  const updateRevision = (
    ev: CustomEvent<{ revision: source.SupportedRevision }>
  ) => {
    source.updateCurrentRevision({ revision: ev.detail.revision });
  };

  // Peers to be displayed in peer selector
  $: availablePeers = revisions && revisions.map(rev => rev.identity);

  // Revisions that belong to the current selected peer, formatted for display in revision selector
  $: currentPeerRevisions =
    revisions && revisions.find(rev => rev.identity.peerId === $currentPeerId);

  $: currentRevision = $currentRevisionStore;
</script>

<style>
  .revision-selector-wrapper {
    width: 18rem;
    position: relative;
    margin-right: 2rem;
  }
</style>

<SidebarLayout dataCy="project-screen">
  <Remote {store} context="project">
    {#if project}
      <Remote store={revisionsStore} context="revisions">
        <Header.Large
          name={project.metadata.name}
          urn={project.shareableEntityIdentifier}
          description={project.metadata.description}
          stats={project.stats}>
          <div slot="left">
            <div style="display: flex">
              <div class="revision-selector-wrapper">
                {#if currentPeerRevisions && currentRevision}
                  <RevisionSelector
                    {currentRevision}
                    revisions={currentPeerRevisions}
                    on:select={updateRevision} />
                {/if}
              </div>
              {#if commits}
                <Remote store={commitsStore}>
                  <HorizontalMenu
                    items={topbarMenuItems(commits.stats.commits)} />
                  <div slot="loading">
                    <HorizontalMenu
                      items={topbarMenuItems()}
                      style="display: inline" />
                  </div>
                </Remote>
              {/if}
            </div>
          </div>
          <div slot="right">
            <CheckoutButton
              on:checkout={ev => project && handleCheckout(ev, project)} />
          </div>
          <div slot="top">
            <div style="display: flex">
              {#if availablePeers && project}
                <PeerSelector
                  {availablePeers}
                  currentPeerId={$currentPeerId}
                  on:select={updatePeer}
                  maintainers={project.metadata.maintainers} />
              {/if}
              <TrackToggle />
            </div>
          </div>
        </Header.Large>
      </Remote>
      <Router {routes} />
    {/if}
  </Remote>
</SidebarLayout>
