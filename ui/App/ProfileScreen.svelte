<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onDestroy } from "svelte";

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

  import { Button, Icon } from "ui/DesignSystem";

  import Header from "ui/App/ScreenLayout/Header.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import Error from "ui/App/ProfileScreen/Error.svelte";
  import ProfileHeader from "ui/App/ProfileScreen/ProfileHeader.svelte";
  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import RequestCardSquare from "ui/App/ProfileScreen/RequestCardSquare.svelte";
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

  const onSelect = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });
  };

  const onUnFollow = (urn: string): void => {
    proxy.client.project.requestCancel(urn).then(fetchProfileProjects);
  };

  const create = () => {
    modal.toggle(CreateProjectModal);
  };

  const projectCardProps = (project: Project) => ({
    title: project.metadata.name,
    description: project.metadata.description || "",
    maintainerUrn: project.metadata.maintainers[0],
    showMaintainerBadge: isMaintainer(session.identity.urn, project),
    anchor: project.anchor,
    stats: project.stats,
    urn: project.urn,
  });

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
    {#if (Array.isArray($profileProjectsStore.data) && $profileProjectsStore.data.length === 0) || $profileProjectsStore.data === null}
      <EmptyState
        text="You don’t have any projects yet."
        primaryActionText="Start your first project"
        on:primaryAction={create} />
    {:else}
      <ul class="grid" data-cy="project-list">
        {#each $profileProjectsStore.data.cloned as project}
          <ProjectCardSquare
            {...projectCardProps(project)}
            on:click={() => onSelect({ detail: project })} />
        {/each}
        {#each $profileProjectsStore.data.follows as project}
          <ProjectCardSquare
            {...projectCardProps(project)}
            on:click={() => onSelect({ detail: project })} />
        {/each}
        {#if $profileProjectsStore.data.requests.length > 0}
          <li class="box requests">
            <div>
              <h2>Still looking...</h2>
              <p style="margin-top: 1rem;">
                {$profileProjectsStore.data.requests.length} project{$profileProjectsStore
                  .data.requests.length > 1
                  ? "s"
                  : ""} you’re following haven’t been found yet.
              </p>
            </div>
            <Button
              variant="outline"
              on:click={() => {
                showRequests = !showRequests;
              }}
              style="align-self: flex-start;">
              {!showRequests ? "Show" : "Hide"}
              {$profileProjectsStore.data.requests.length} project{$profileProjectsStore
                .data.requests.length > 1
                ? "s"
                : ""}
            </Button>
          </li>
          {#if showRequests}
            {#each $profileProjectsStore.data.requests as project}
              <RequestCardSquare
                urn={project.urn}
                on:unfollow={() => onUnFollow(project.urn)} />
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
