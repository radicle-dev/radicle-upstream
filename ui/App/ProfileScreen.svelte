<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onDestroy } from "svelte";

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
  import ProfileHeader from "./ProfileScreen/ProfileHeader.svelte";
  import ProjectCardSquare from "./ProfileScreen/ProjectCardSquare.svelte";

  import CreateProjectModal from "ui/App/CreateProjectModal.svelte";

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

  const projectCardProps = (project: Project) => ({
    title: project.metadata.name,
    description: project.metadata.description || "",
    maintainerUrn: project.metadata.maintainers[0],
    showMaintainerBadge: isMaintainer(session.identity.urn, project),
    anchor: project.anchor,
    stats: project.stats,
    urn: project.urn,
  });

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
    <ul class="grid">
      {#each $profileProjectsStore.data.cloned as project}
        <ProjectCardSquare {...projectCardProps(project)} />
      {/each}
      {#each $profileProjectsStore.data.follows as project}
        <ProjectCardSquare {...projectCardProps(project)} />
      {/each}
      {#each $profileProjectsStore.data.requests as project}
        requests
      {/each}
    </ul>
  {/if}
</ScreenLayout>
