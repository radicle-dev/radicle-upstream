<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as proxyIdentity from "proxy-client/identity";
  import type * as proxyProject from "proxy-client/project";

  import { isDelegate, Project } from "ui/src/project";
  import * as router from "ui/src/router";
  import * as svelteStore from "ui/src/svelteStore";
  import * as ethereum from "ui/src/ethereum";
  import * as Wallet from "ui/src/wallet";
  import * as graph from "ui/src/org/theGraphApi";
  import {
    getCachedRegistrationByAddress,
    getRegistration,
    Registration,
  } from "ui/src/org/ensResolver";
  import type { Org } from "ui/src/org";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import UserProfileHeader from "./UserProfileScreen/UserProfileHeader.svelte";
  import ProfileSidebar from "ui/App/ProfileScreen/ProfileSidebar.svelte";

  export let projects: proxyProject.Project[];
  export let user: proxyIdentity.RemoteIdentity;
  export let ownUserUrn: string;

  function openProject(project: Project): void {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
  }

  let registration: Registration | undefined;
  let ownedOrgs: Org[] = [];

  const wallet = svelteStore.get(Wallet.store);
  const ethereumEnvironment = ethereum.selectedEnvironment;

  async function loadSidebarData(): Promise<void> {
    const address = user.metadata.ethereum?.address;
    if (!address) {
      return;
    }

    const gnosisSafeWallets = await graph.getSafesByOwner(address);
    ownedOrgs = await graph.getOwnedOrgs([
      address,
      ...gnosisSafeWallets.map(safe => safe.id),
    ]);
    if (ownedOrgs) {
      ownedOrgs.map(async org => {
        const registration = await getCachedRegistrationByAddress(org.id);
        if (registration) {
          org.registration = registration;
        }
        return org;
      });
    }

    const ensName = await wallet.provider.lookupAddress(address);
    if (!ensName) {
      return;
    }
    registration = await getRegistration(ensName);
  }

  loadSidebarData();

  $: showSidebar =
    $wallet.status === Wallet.Status.Connected &&
    ethereum.supportedNetwork($ethereumEnvironment) ===
      $wallet.connected.network &&
    user.metadata.ethereum?.address !== undefined;
</script>

<style>
  .sidebar-layout {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    grid-template-rows: 15rem;
    gap: 1.5rem;
    grid-template-areas: "main main sidebar";
  }

  .one-column {
    grid-template-areas: "main main main";
  }

  .sidebar {
    grid-area: sidebar;
  }

  .grid {
    grid-area: main;
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
  }

  .two-columns {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .empty {
    grid-column: 1 / span 2;
  }

  .three-columns {
    grid-column: 1 / span 3;
  }
</style>

<ScreenLayout dataCy="user-profile-screen">
  <div slot="header" style="display: flex">
    <UserProfileHeader {user} />
  </div>

  <div class="sidebar-layout" class:one-column={!showSidebar}>
    {#if projects.length === 0}
      <div class="empty" class:three-columns={!showSidebar}>
        <EmptyState text="This peer doesn't have any projects." />
      </div>
    {:else}
      <ul class="grid" data-cy="project-list" class:two-columns={showSidebar}>
        {#each projects as project}
          <li>
            <ProjectCardSquare
              {project}
              isDelegate={isDelegate(ownUserUrn, project)}
              on:click={() => openProject(project)} />
          </li>
        {/each}
      </ul>
    {/if}
    {#if showSidebar}
      <div class="sidebar">
        <ProfileSidebar
          attestedAddress={user.metadata.ethereum?.address}
          {registration}
          {ownedOrgs}
          urn={user.urn} />
      </div>
    {/if}
  </div>
</ScreenLayout>
