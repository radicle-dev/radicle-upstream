<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { isMaintainer, Project } from "ui/src/project";
  import * as router from "ui/src/router";
  import * as Session from "ui/src/session";
  import * as remote from "ui/src/remote";
  import * as proxy from "ui/src/proxy";
  import type * as proxyProject from "proxy-client/project";
  import type * as proxyIdentity from "proxy-client/identity";
  import * as error from "ui/src/error";

  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import Error from "ui/App/ProfileScreen/Error.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";

  import UserProfileHeader from "./UserProfileScreen/UserProfileHeader.svelte";

  export let urn: string;

  function openProject(project: Project) {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
  }

  const userStore = remote.createStore<proxyIdentity.RemoteIdentity>();
  fetchUser();

  const projectsStore = remote.createStore<proxyProject.Project[]>();
  remote.fetch(projectsStore, proxy.client.project.listForUser(urn));

  const session = Session.unsealed();

  async function fetchUser() {
    try {
      userStore.success(await proxy.client.personGet(urn));
    } catch (err: unknown) {
      error.show(
        new error.Error({
          message: "Failed to fetch user data",
          source: err,
        })
      );
    }
  }
</script>

<style>
  .grid {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    margin: 0 auto;
    padding: 2rem var(--content-padding);
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
  }
</style>

<ScreenLayout dataCy="user-profile-screen">
  {#if $userStore.status === remote.Status.Success}
    <Header>
      <UserProfileHeader
        slot="left"
        identityMetadata={$userStore.data.metadata}
        deviceIds={$userStore.data.peerIds}
        {urn} />
    </Header>

    {#if $projectsStore.status === remote.Status.Success}
      {#if $projectsStore.data.length === 0}
        <EmptyState text="This peer doesn't have any projects." />
      {:else}
        <ul class="grid" data-cy="project-list">
          {#each $projectsStore.data as project}
            <li>
              <ProjectCardSquare
                {project}
                isMaintainer={isMaintainer(session.identity.urn, project)}
                on:click={() => openProject(project)} />
            </li>
          {/each}
        </ul>
      {/if}
    {:else if $projectsStore.status === remote.Status.Error}
      <Error message={$projectsStore.error.message} />
    {/if}
  {/if}
</ScreenLayout>
