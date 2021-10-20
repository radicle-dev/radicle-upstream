<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as proxyIdentity from "proxy-client/identity";
  import type * as proxyProject from "proxy-client/project";

  import { isMaintainer, Project } from "ui/src/project";
  import * as router from "ui/src/router";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import UserProfileHeader from "./UserProfileScreen/UserProfileHeader.svelte";

  export let projects: proxyProject.Project[];
  export let user: proxyIdentity.RemoteIdentity;
  export let ownUserUrn: string;

  function openProject(project: Project) {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
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
  <Header>
    <UserProfileHeader slot="left" {user} />
  </Header>

  {#if projects.length === 0}
    <EmptyState text="This peer doesn't have any projects." />
  {:else}
    <ul class="grid" data-cy="project-list">
      {#each projects as project}
        <li>
          <ProjectCardSquare
            {project}
            isMaintainer={isMaintainer(ownUserUrn, project)}
            on:click={() => openProject(project)} />
        </li>
      {/each}
    </ul>
  {/if}
</ScreenLayout>
