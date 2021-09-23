<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as Session from "ui/src/session";
  import * as modal from "ui/src/modal";
  import * as remote from "ui/src/remote";
  import * as proxy from "ui/src/proxy";
  import * as error from "ui/src/error";
  import * as project from "ui/src/project";

  import { Avatar, Button, Icon } from "ui/DesignSystem";

  import Header from "ui/App/ScreenLayout/Header.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ProfileHeader from "./ProfileScreen/ProfileHeader.svelte";

  import CreateProjectModal from "ui/App/CreateProjectModal.svelte";

  const session = Session.unsealed();

  // TYPES
  interface ProfileProjects {
    cloned: project.Project[];
    follows: project.Project[];
    requests: project.Request[];
  }

  const profileProjectsStore = remote.createStore<ProfileProjects>();

  const fetchProfileProjects = async (): Promise<void> => {
    profileProjectsStore.loading();

    try {
      const cloned = await proxy.client.project.listContributed();
      const follows = await proxy.client.project.listTracked();
      const allRequests = await proxy.client.project.requestsList();
      const requests = allRequests.filter(
        req =>
          req.type !== project.RequestStatus.Cloned &&
          req.type !== project.RequestStatus.Cancelled &&
          req.type !== project.RequestStatus.TimedOut
      );

      profileProjectsStore.success({ cloned, follows, requests });
    } catch (err: unknown) {
      error.show(
        new error.Error({ message: "Failed to fetch projects.", source: err })
      );
    }
  };

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

  fetchProfileProjects();
  showNotificationsForFailedProjects();
</script>

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
    <ul>
      <h1>cloned</h1>
      {#each $profileProjectsStore.data.cloned as project}
        <li>
          <Avatar
            style="margin-right: 0.625rem;"
            size="small"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: project.metadata.maintainers[0],
            }} />
          <pre>
            {JSON.stringify(project, null, 2)}
          </pre>
        </li>
      {/each}
      <h1>follow</h1>
      {#each $profileProjectsStore.data.follows as project}
        <li>
          <Avatar
            style="margin-right: 0.625rem;"
            size="small"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: project.metadata.maintainers[0],
            }} />
          <pre>
            {JSON.stringify(project, null, 2)}
          </pre>
        </li>
      {/each}
      <h1>request</h1>
      {#each $profileProjectsStore.data.requests as project}
        <li>
          <pre>
            {JSON.stringify(project, null, 2)}
          </pre>
        </li>
      {/each}
    </ul>
  {/if}
</ScreenLayout>
