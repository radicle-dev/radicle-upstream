<script lang="typescript">
  import { getContext } from "svelte";
  import Router from "svelte-spa-router";

  import { isExperimental, openPath } from "../../native/ipc.js";

  import * as menu from "../src/menu";
  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import {
    checkout,
    fetch,
    isMaintainer,
    project as store,
    peerSelection,
    revisionSelection,
    selectPeer,
    selectedPeer,
  } from "../src/project";
  import type { Project, User } from "../src/project";
  import type { UnsealedSession } from "../src/session";
  import * as screen from "../src/screen";
  import {
    commits as commitsStore,
    currentRevision,
    resetCurrentRevision,
    resetCurrentPeerId,
    RevisionType,
  } from "../src/source";
  import { CSSPosition } from "../src/style";
  import type { Urn } from "../src/urn";

  import {
    FollowToggle,
    Header,
    HorizontalMenu,
    Remote,
    RevisionSelector,
    SidebarLayout,
    Tooltip,
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
    "/projects/:urn/": Source,
    "/projects/:urn/source": Source,
    "/projects/:urn/issues": Issues,
    "/projects/:urn/issue": Issue,
    "/projects/:urn/commit/:hash": Commit,
    "/projects/:urn/commits": Commits,
    "/projects/:urn/revisions": Revisions,
  };

  export let params: { urn: Urn };

  const urn = params.urn;
  const session: UnsealedSession = getContext("session");
  const trackTooltipMaintainer = "You can't unfollow your own project";
  const trackTooltip = "Unfollowing is not yet supported";

  // Reset some stores on first load
  resetCurrentRevision();
  resetCurrentPeerId();

  $: topbarMenuItems = (
    project: Project,
    commitCounter?: number
  ): menu.HorizontalItem[] => {
    const items = [
      {
        icon: Icon.House,
        title: "Source",
        href: path.projectSource(project.urn),
        looseActiveStateMatching: true,
      },
      {
        icon: Icon.Commit,
        title: "Commits",
        counter: commitCounter,
        href: path.projectCommits(project.urn),
        looseActiveStateMatching: true,
      },
    ];
    isExperimental() &&
      items.push(
        {
          icon: Icon.ExclamationCircle,
          title: "Issues",
          href: path.projectIssues(urn),
          looseActiveStateMatching: false,
        },
        {
          icon: Icon.Revision,
          title: "Revisions",
          href: path.projectRevisions(urn),
          looseActiveStateMatching: false,
        }
      );
    return items;
  };

  const handleCheckout = async (
    { detail: checkoutDirectoryPath }: CustomEvent,
    project: Project,
    peer: User | null
  ) => {
    if (peer === null) {
      notification.error(`Can't checkout without a peer selected`);
      return;
    }

    try {
      screen.lock();
      const path = await checkout(
        project.urn,
        checkoutDirectoryPath,
        peer.identity.peerId
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
  const onSelectPeer = ({ detail: peer }: { detail: User }) => {
    selectPeer(peer);
  };
  const selectRevision = (event: CustomEvent) => {
    console.log(event);
  };

  // Initialise the screen by fetching the project and associated data.
  fetch({ urn });
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
    <Header.Large urn={project.urn} stats={project.stats} {...project.metadata}>
      <div slot="top">
        <div style="display: flex">
          <Remote store={peerSelection} let:data>
            {#if data.peers.length > 0}
              <PeerSelector
                peers={data.peers}
                on:select={onSelectPeer}
                selected={$selectedPeer || data.default} />
              <Tooltip
                position={CSSPosition.Left}
                value={isMaintainer(session.identity.urn, project) ? trackTooltipMaintainer : trackTooltip}>
                <FollowToggle disabled following />
              </Tooltip>
            {/if}
          </Remote>
        </div>
      </div>
      <div slot="left">
        <div style="display: flex">
          <Remote store={revisionSelection} let:data={revisions}>
            <div class="revision-selector-wrapper">
              <RevisionSelector
                on:select={selectRevision}
                selected={$currentRevision || { type: RevisionType.Branch, name: project.metadata.defaultBranch }}
                {revisions} />
            </div>
          </Remote>
          <Remote store={commitsStore} let:data={commits}>
            <HorizontalMenu
              items={topbarMenuItems(project, commits.stats.commits)} />
            <div slot="loading">
              <HorizontalMenu
                items={topbarMenuItems(project)}
                style="display: inline" />
            </div>
          </Remote>
        </div>
      </div>
      <div slot="right">
        <CheckoutButton
          on:checkout={ev => handleCheckout(ev, project, $selectedPeer)} />
      </div>
    </Header.Large>
    <Router {routes} />
  </Remote>
</SidebarLayout>
