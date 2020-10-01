<script lang="ts">
  import { isExperimental, openPath } from "../../native/ipc.js";
  import Router from "svelte-spa-router";

  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import { checkout, fetch, project as projectStore } from "../src/project";
  import type { Project } from "../src/project";
  import * as remote from "../src/remote";
  import * as screen from "../src/screen";
  import {
    commits as commitsStore,
    currentRevision as currentRevisionStore,
    currentPeerId,
    fetchCommits,
    fetchRevisions,
    resetCurrentRevision,
    resetCurrentPeerId,
    revisions as revisionsStore,
  } from "../src/source";

  import type { SupportedRevision } from "../src/source";

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

  let currentRevision: SupportedRevision | undefined;

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
  $: if (currentRevision)
    fetchCommits({ projectId, revision: currentRevision });

  $: ps = $projectStore;
  $: project = ps.status === remote.Status.Success && ps.data;

  $: rs = $revisionsStore;
  $: revisions = rs.status === remote.Status.Success && rs.data;

  $: cs = $commitsStore;
  $: commits = cs.status === remote.Status.Success && cs.data;

  $: currentRevisionStore.set(currentRevision);
</script>

<style>
  .revision-selector-wrapper {
    width: 18rem;
    position: relative;
    margin-right: 2rem;
  }
</style>

<SidebarLayout dataCy="project-screen">
  <Remote store={projectStore} context="project">
    <Remote store={revisionsStore} context="revisions">
      {#if project}
        <Header.Large
          name={project.metadata.name}
          urn={project.shareableEntityIdentifier}
          description={project.metadata.description}
          stats={project.stats}>
          <div slot="left">
            <div style="display: flex">
              <div class="revision-selector-wrapper">
                {#if $currentPeerId && revisions}
                  <RevisionSelector
                    currentPeerId={$currentPeerId}
                    bind:currentRevision
                    {revisions} />
                {/if}
              </div>
              <Remote store={commitsStore}>
                {#if commits && project}
                  <HorizontalMenu
                    items={topbarMenuItems(project, commits.stats.commits)} />
                {/if}

                <div slot="loading">
                  <HorizontalMenu
                    items={topbarMenuItems(project, undefined)}
                    style="display: inline" />
                </div>
              </Remote>
            </div>
          </div>
          <div slot="right">
            <CheckoutButton
              on:checkout={ev => project && handleCheckout(ev, project)} />
          </div>
          <div slot="top">
            <div style="display: flex">
              <PeerSelector
                bind:currentPeerId={$currentPeerId}
                {revisions}
                on:select={() => {
                  resetCurrentRevision();
                }} />
              <TrackToggle />
            </div>
          </div>
        </Header.Large>
      {/if}
    </Remote>
    <Router {routes} />
  </Remote>
</SidebarLayout>
