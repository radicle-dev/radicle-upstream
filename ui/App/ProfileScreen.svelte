<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project } from "ui/src/project";
  import type { Org } from "ui/src/org";

  import { onDestroy } from "svelte";
  import { fade } from "svelte/transition";

  import { isDelegate } from "ui/src/project";
  import * as Session from "ui/src/session";
  import * as error from "ui/src/error";
  import * as localPeer from "ui/src/localPeer";
  import * as svelteStore from "ui/src/svelteStore";
  import * as ethereum from "ui/src/ethereum";
  import * as Wallet from "ui/src/wallet";
  import { getRegistration, Registration } from "ui/src/org/ensResolver";
  import { orgSidebarStore } from "ui/src/org";
  import * as modal from "ui/src/modal";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import * as project from "ui/src/project";
  import * as proxy from "ui/src/proxy";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";

  import PlusIcon from "design-system/icons/Plus.svelte";
  import MagnifyingGlassIcon from "design-system/icons/MagnifyingGlass.svelte";
  import Button from "design-system/Button.svelte";
  import TrackToggle from "design-system/TrackToggle.svelte";

  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import Error from "ui/App/ProfileScreen/Error.svelte";
  import ProfileHeader from "ui/App/ProfileScreen/ProfileHeader.svelte";
  import ProfileSidebar from "ui/App/ProfileScreen/ProfileSidebar.svelte";
  import ProjectCardSquare from "ui/App/ProfileScreen/ProjectCardSquare.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import SearchModal from "ui/App/SearchModal.svelte";

  let registration: Registration | undefined;
  let ownedOrgs: Org[] = [];
  const ethereumEnvironment = ethereum.selectedEnvironment;
  const session = Session.unsealed();

  interface ProfileProjects {
    cloned: project.Project[];
    tracked: project.Project[];
    requests: project.Request[];
  }

  const profileProjectsStore = remote.createStore<ProfileProjects>();
  profileProjectsStore.loading();

  const loadProfileProjectsExecutor = mutexExecutor.create();

  loadProfileProjects();

  const unsubscribeProjectEvents = localPeer.requestEvents
    .merge(localPeer.projectEvents)
    .subscribe(() => {
      loadProfileProjects();
    });

  onDestroy(() => {
    unsubscribeProjectEvents();
  });

  async function loadProfileProjects(): Promise<void> {
    try {
      const profileProjects = await loadProfileProjectsExecutor.run(
        fetchProfileProjects
      );
      if (profileProjects) {
        profileProjectsStore.success(profileProjects);
      }
    } catch (err: unknown) {
      notification.showException(
        new error.Error({ message: "Failed to fetch projects.", source: err })
      );
    }
  }

  async function fetchProfileProjects(): Promise<ProfileProjects> {
    const [cloned, tracked, allRequests] = await Promise.all([
      proxy.client.project.listContributed(),
      proxy.client.project.listTracked(),
      proxy.client.project.requestsList(),
    ]);
    const urns = [...cloned, ...tracked].map(p => p.urn);

    const requests = allRequests.filter(
      req =>
        req.type !== project.RequestStatus.Cloned &&
        req.type !== project.RequestStatus.Cancelled &&
        req.type !== project.RequestStatus.TimedOut &&
        !urns.includes(req.urn)
    );

    return { cloned, tracked, requests };
  }

  const showNotificationsForFailedProjects = async (): Promise<void> => {
    try {
      const failedProjects = await proxy.client.project.listFailed();
      if (failedProjects.length > 0) {
        notification.showException(
          new error.Error({
            message: "Some of your projects could not be loaded",
            details: failedProjects,
          })
        );
      }
    } catch (err: unknown) {
      notification.showException(
        new error.Error({
          code: error.Code.ProjectRequestFailure,
          message: "Failed to get failed projects",
          source: err,
        })
      );
    }
  };

  function openProject({ detail: project }: { detail: Project }): void {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
  }

  function onUntrack(urn: string): void {
    proxy.client.project.requestCancel(urn).then(loadProfileProjects);
  }

  function projectCountText(storeLength: number): string {
    return `${storeLength} project${storeLength > 1 ? "s" : ""}`;
  }

  let showRequests: boolean = false;
  showNotificationsForFailedProjects();

  const wallet = svelteStore.get(Wallet.store);

  async function loadSidebarData(): Promise<void> {
    const address =
      session.identity.metadata.ethereum?.address || Wallet.walletAddress();
    if (!address) {
      return;
    }

    const ensName = await wallet.provider.lookupAddress(address);
    if (!ensName) {
      return;
    }
    registration = await getRegistration(ensName);
  }

  loadSidebarData();

  $: if ($orgSidebarStore.type === "resolved") {
    ownedOrgs = $orgSidebarStore.orgs;
  }

  $: showSidebar =
    ($wallet.status === Wallet.Status.Connected &&
      ethereum.supportedNetwork($ethereumEnvironment) ===
        $wallet.connected.network &&
      session.identity.metadata.ethereum?.address !== undefined) ||
    Wallet.walletAddress() !== undefined;
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

  .box {
    border: 1px solid var(--color-foreground-level-2);
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
  }

  .search {
    justify-content: center;
    align-items: center;
  }
</style>

<ScreenLayout dataCy="profile-screen">
  <div slot="header" style="display: flex">
    <ProfileHeader
      urn={session.identity.urn}
      name={registration?.domain
        ? registration?.domain
        : session.identity.metadata.handle}
      peerId={session.identity.peerId} />

    <CommandModal
      let:prop={toggleDropdown}
      command={"rad init"}
      description="To initialize a Radicle project, run the following command in an existing repository:">
      <Button variant="outline" icon={PlusIcon} on:click={toggleDropdown}>
        New project
      </Button>
    </CommandModal>
  </div>

  {#if $profileProjectsStore.status === remote.Status.Success}
    <div class="sidebar-layout" class:one-column={!showSidebar}>
      {#if $profileProjectsStore.data.cloned.length === 0 && $profileProjectsStore.data.tracked.length === 0 && $profileProjectsStore.data.requests.length === 0}
        <div class="empty" class:three-columns={!showSidebar}>
          <EmptyState text="You haven’t created or tracked any projects yet.">
            <CommandModal
              let:prop={onClickhandler}
              command={"rad init"}
              description="To initialize a Radicle project, run the following command in an existing repository:">
              <Button on:click={onClickhandler} dataCy="primary-action">
                Start your first project
              </Button>
            </CommandModal>
            <button
              class="typo-link"
              style="margin-top: 0.5rem;"
              on:click={() => {
                modal.toggle(SearchModal);
              }}>
              Or look for a project
            </button>
          </EmptyState>
        </div>
      {:else}
        <ul class="grid" data-cy="project-list" class:two-columns={showSidebar}>
          {#each $profileProjectsStore.data.cloned as project}
            <li>
              <ProjectCardSquare
                {project}
                isDelegate={isDelegate(session.identity.urn, project)}
                on:click={() => openProject({ detail: project })} />
            </li>
          {/each}
          {#each $profileProjectsStore.data.tracked as project}
            <li>
              <ProjectCardSquare
                isDelegate={isDelegate(session.identity.urn, project)}
                {project}
                on:click={() => openProject({ detail: project })} />
            </li>
          {/each}
          {#if $profileProjectsStore.data.requests.length > 0}
            <li class="box requests">
              <div>
                <h2>Still looking...</h2>
                <p style="margin-top: 1rem;">
                  {projectCountText($profileProjectsStore.data.requests.length)}
                  {$profileProjectsStore.data.requests.length > 1
                    ? `you’re tracking haven’t been found yet.`
                    : `you’re tracking hasn't been found yet.`}
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
                  <CopyableIdentifier kind="projectId" value={project.urn} />
                  <TrackToggle
                    style="align-self: flex-start;"
                    tracking
                    on:untrack={() => onUntrack(project.urn)} />
                </li>
              {/each}
            {/if}
          {/if}
          <li class="search box" data-cy="search-box">
            <p
              style="color: var(--color-foreground-level-5); margin-bottom: 1.5rem;">
              Track a new project
            </p>
            <Button
              on:click={() => {
                modal.toggle(SearchModal);
              }}
              icon={MagnifyingGlassIcon}
              variant="outline">
              Look for a project
            </Button>
          </li>
        </ul>
      {/if}
      {#if showSidebar}
        <div class="sidebar">
          <ProfileSidebar
            attestedAddress={session.identity.metadata.ethereum?.address}
            {registration}
            {ownedOrgs}
            urn={session.identity.urn} />
        </div>
      {/if}
    </div>
  {:else if $profileProjectsStore.status === remote.Status.Error}
    <Error message={$profileProjectsStore.error.message} />
  {/if}
</ScreenLayout>
