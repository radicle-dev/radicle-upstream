<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onDestroy } from "svelte";
  import { fade } from "svelte/transition";

  import * as router from "ui/src/router";
  import * as Session from "ui/src/session";
  import * as modal from "ui/src/modal";
  import * as remote from "ui/src/remote";
  import * as proxy from "ui/src/proxy";
  import * as error from "ui/src/error";
  import type { Project } from "ui/src/project";
  import { isMaintainer } from "ui/src/project";
  import * as project from "ui/src/project";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as localPeer from "ui/src/localPeer";

  import {
    Button,
    Icon,
    FollowToggle,
    CopyableIdentifier,
  } from "ui/DesignSystem";

  import Header from "ui/App/ScreenLayout/Header.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import Error from "ui/App/ProfileScreen/Error.svelte";
  import ProfileHeader from "ui/App/ProfileScreen/ProfileHeader.svelte";
  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import SearchModal from "ui/App/SearchModal.svelte";
  import CreateProjectModal from "ui/App/CreateProjectModal.svelte";
  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";

  const session = Session.unsealed();

  interface ProfileProjects {
    cloned: project.Project[];
    follows: project.Project[];
    requests: project.Request[];
  }

  const profileProjectsStore = remote.createStore<ProfileProjects>();
  profileProjectsStore.loading();

  const fetchProfileProjectsExecutor = mutexExecutor.create();

  fetchProfileProjects();
  const unsubPeerEvents = localPeer.requestEvents.subscribe(() => {
    fetchProfileProjects();
  });
  onDestroy(unsubPeerEvents);

  async function fetchProfileProjects(): Promise<void> {
    try {
      const profileProjects = await fetchProfileProjectsExecutor.run(
        async () => {
          const [cloned, follows, allRequests] = await Promise.all([
            proxy.client.project.listContributed(),
            proxy.client.project.listTracked(),
            proxy.client.project.requestsList(),
          ]);
          const requests = allRequests.filter(
            req =>
              req.type !== project.RequestStatus.Cloned &&
              req.type !== project.RequestStatus.Cancelled &&
              req.type !== project.RequestStatus.TimedOut
          );
          return { cloned, follows, requests };
        }
      );
      if (profileProjects) {
        profileProjectsStore.success(profileProjects);
      }
    } catch (err: unknown) {
      error.show(
        new error.Error({ message: "Failed to fetch projects.", source: err })
      );
    }
  }

  const showNotificationsForFailedProjects = async (): Promise<void> => {
    try {
      const failedProjects = await proxy.client.project.listFailed();
      failedProjects.forEach(failedProject => {
        error.show(
          new error.Error({
            code: error.Code.ProjectRequestFailure,
            message: `The project ${failedProject.metadata.name} couldn’t be loaded`,
            details: failedProject,
          })
        );
      });
    } catch (err: unknown) {
      error.show(
        new error.Error({
          code: error.Code.ProjectRequestFailure,
          message: "Failed to get failed projects",
          source: err,
        })
      );
    }
  };

  function openProject({ detail: project }: { detail: Project }) {
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });
  }

  function onUnFollow(urn: string): void {
    proxy.client.project.requestCancel(urn).then(fetchProfileProjects);
  }

  function projectCountText(storeLength: number) {
    return `${storeLength} project${storeLength > 1 ? "s" : ""}`;
  }

  let showRequests: boolean = false;

  showNotificationsForFailedProjects();
</script>

<style>
  .grid {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    margin: 0 auto;
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
    padding: 2rem;
  }

  .box {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    padding: 2rem;
    height: 15rem;
    display: flex;
    flex-direction: column;
  }

  .requests {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .request-card {
    justify-content: space-between;
    cursor: pointer;
  }

  .search {
    justify-content: center;
    align-items: center;
  }
</style>

<ScreenLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header>
    <ProfileHeader
      slot="left"
      urn={session.identity.urn}
      name={session.identity.metadata.handle}
      peerId={session.identity.peerId} />

    <Button
      slot="right"
      dataCy="new-project-button"
      variant="outline"
      icon={Icon.Plus}
      on:click={() => {
        modal.toggle(CreateProjectModal);
      }}>
      New project
    </Button>
  </Header>

  {#if $profileProjectsStore.status === remote.Status.Success}
    {#if Array.isArray($profileProjectsStore.data) && $profileProjectsStore.data.length === 0}
      <EmptyState
        text="You don’t have any projects yet."
        primaryActionText="Start your first project"
        on:primaryAction={() => modal.toggle(CreateProjectModal)} />
    {:else}
      <ul class="grid" data-cy="project-list">
        {#each $profileProjectsStore.data.cloned as project}
          <li>
            <ProjectCardSquare
              {project}
              isMaintainer={isMaintainer(session.identity.urn, project)}
              on:click={() => openProject({ detail: project })} />
          </li>
        {/each}
        {#each $profileProjectsStore.data.follows as project}
          <li>
            <ProjectCardSquare
              isMaintainer={isMaintainer(session.identity.urn, project)}
              {project}
              on:click={() => openProject({ detail: project })} />
          </li>
        {/each}
        {#if $profileProjectsStore.data.requests.length > 0}
          <li class="box requests">
            <div>
              <h2>Still looking...</h2>
              <p style="margin-top: 1rem;">
                {projectCountText($profileProjectsStore.data.requests.length)} you’re
                following haven’t been found yet.
              </p>
            </div>
            <Button
              variant="outline"
              dataCy="show-requests"
              on:click={() => {
                showRequests = !showRequests;
              }}
              style="align-self: flex-start;">
              {!showRequests ? "Show" : "Hide"}
              {projectCountText($profileProjectsStore.data.requests.length)}
            </Button>
          </li>
          {#if showRequests}
            {#each $profileProjectsStore.data.requests as project}
              <li
                class="request-card box"
                data-cy="undiscovered-project"
                out:fade|local={{ duration: 200 }}>
                <CopyableIdentifier kind="radicleId" value="{project.urn}}" />
                <FollowToggle
                  style="align-self: flex-start;"
                  following
                  on:unfollow={() => onUnFollow(project.urn)} />
              </li>
            {/each}
          {/if}
        {/if}
        <li class="search box" data-cy="search-box">
          <p
            style="color: var(--color-foreground-level-5); margin-bottom: 1.5rem;">
            Follow a new project
          </p>
          <Button
            on:click={() => {
              modal.toggle(SearchModal);
            }}
            icon={Icon.MagnifyingGlass}
            variant="outline">
            Look for a project
          </Button>
        </li>
      </ul>
    {/if}
  {:else if $profileProjectsStore.status === remote.Status.Error}
    <Error message={$profileProjectsStore.error.message} />
  {/if}
</ScreenLayout>
