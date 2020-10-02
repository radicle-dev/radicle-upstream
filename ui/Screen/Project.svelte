<script lang="ts">
  import Router from "svelte-spa-router";
  import { isExperimental, openPath } from "../../native/ipc.js";

  import * as identity from "../src/identity";
  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import { checkout, fetch, project as store } from "../src/project";
  import type { Project } from "../src/project";
  import * as remote from "../src/remote";
  import * as screen from "../src/screen";
  import {
    commits as commitsStore,
    currentPeerId as currentPeerIdStore,
    currentRevision as currentRevisionStore,
    fetchCommits,
    fetchRevisions,
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

  // Reset some stores on first load
  resetCurrentRevision();
  resetCurrentPeerId();

  $: topbarMenuItems = (project: Project, commitCounter?: number) => {
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

  const handleCheckout = async (
    event: CustomEvent<{ checkoutDirectoryPath: string }>,
    project: Project
  ) => {
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
  fetchRevisions({ projectId });

  let project: Project | undefined;
  let revisions: source.PeerRevisions[] | undefined;

  $: {
    const revStore = $revisionsStore;
    revisions =
      revStore.status === remote.Status.Success ? revStore.data : undefined;

    const pStore = $store;
    project = pStore.status === remote.Status.Success ? pStore.data : undefined;
  }

  let currentPeerId: string,
    currentPeerRevisions: source.PeerRevisions,
    currentRevision: source.Branch | source.Tag,
    availablePeers: identity.Identity[];

  // Initialize available peers & revisions once revisions have loaded
  $: {
    if (revisions && project) {
      // If no peer has been selected, choose the first one
      if (!currentPeerId) currentPeerId = revisions[0].identity.peerId;
      currentPeerIdStore.set(currentPeerId);

      // Now we can get the revisions for the current peer
      currentPeerRevisions = revisions.filter(
        rev => rev.identity.peerId === currentPeerId
      )[0];

      const defaultBranch = project.metadata.defaultBranch;

      // Now that we have a peer, set the current revision to the default branch.
      // If not found, use the first branch returned from proxy.
      if (!currentRevision)
        currentRevision =
          currentPeerRevisions.branches.find(
            branch => branch.name === defaultBranch
          ) || currentPeerRevisions.branches[0];

      currentRevisionStore.set(currentRevision);

      // Peers to be displayed in peer selector UI
      availablePeers = revisions.map(rev => rev.identity);
    }
  }

  const updatePeer = (ev: CustomEvent<{ peerId: string }>) => {
    const newPeer =
      availablePeers &&
      (availablePeers.find(peer => peer.peerId === ev.detail.peerId) ||
        availablePeers[0]);

    if (newPeer) {
      currentPeerId = newPeer.peerId;
      currentPeerIdStore.set(currentPeerId);
    }
  };

  const updateRevision = (
    ev: CustomEvent<{ revision: source.Branch | source.Tag }>
  ) => {
    console.log("updating revision", ev);
    currentRevision = ev.detail.revision;
    currentRevisionStore.set(currentRevision);
  };

  $: if (currentRevision) {
    fetchCommits({ projectId, revision: currentRevision });
  }
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
    <Remote store={revisionsStore} context="revisions">
      <Header.Large
        name={project.metadata.name}
        urn={project.shareableEntityIdentifier}
        description={project.metadata.description}
        stats={project.stats}>
        <div slot="left">
          <div style="display: flex">
            <div class="revision-selector-wrapper">
              {#if currentPeerId}
                <RevisionSelector
                  {currentRevision}
                  revisions={currentPeerRevisions}
                  on:select={updateRevision} />
              {/if}
            </div>
            <Remote store={commitsStore} let:data={commits}>
              <HorizontalMenu
                items={topbarMenuItems(project, commits.stats.commits)} />
              <div slot="loading">
                <HorizontalMenu
                  items={topbarMenuItems(project, undefined)}
                  style="display: inline" />
              </div>
            </Remote>
          </div>
        </div>
        <div slot="right">
          <CheckoutButton on:checkout={ev => handleCheckout(ev, project)} />
        </div>
        <div slot="top">
          <div style="display: flex">
            {#if availablePeers}
              <PeerSelector
                {availablePeers}
                {currentPeerId}
                on:select={updatePeer} />
            {/if}
            <TrackToggle />
          </div>
        </div>
      </Header.Large>
    </Remote>
    <Router {routes} />
  </Remote>
</SidebarLayout>
