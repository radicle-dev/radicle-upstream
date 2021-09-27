<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";
  import { fetchProjects, projects as store } from "ui/src/userProfile";
  import { isMaintainer } from "ui/src/project";
  import * as router from "ui/src/router";
  import * as Session from "ui/src/session";
  import * as remote from "ui/src/remote";
  import * as userProfile from "ui/src/userProfile";

  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import Error from "ui/App/ProfileScreen/Error.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";

  import UserProfileHeader from "./UserProfileScreen/UserProfileHeader.svelte";

  export let urn: string;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });
  };

  const userProfileStore = userProfile.user;
  const session = Session.unsealed();

  fetchProjects(urn);
  userProfile.fetchUser(urn);
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

<ScreenLayout dataCy="user-profile-screen">
  {#if $userProfileStore.status === remote.Status.Success}
    <Header>
      <UserProfileHeader
        slot="left"
        identityMetadata={$userProfileStore.data.metadata}
        deviceIds={$userProfileStore.data.peerIds}
        {urn} />
    </Header>

    {#if $store.status === remote.Status.Success}
      {#if (Array.isArray($store.data) && $store.data.length === 0) || $store.data === null}
        <EmptyState text="This peer doesn't have any projects." />
      {:else}
        <ul class="grid" data-cy="project-list">
          {#each $store.data as project}
            <li>
              <ProjectCardSquare
                {project}
                isMaintainer={isMaintainer(session.identity.urn, project)}
                on:click={() => select({ detail: project })} />
            </li>
          {/each}
        </ul>
      {/if}
    {:else if $store.status === remote.Status.Error}
      <Error message={$store.error.message} />
    {/if}
  {/if}
</ScreenLayout>
