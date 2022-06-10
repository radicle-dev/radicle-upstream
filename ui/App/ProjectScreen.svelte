<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { ConfirmedAnchor, User, Project } from "ui/src/project";

  import { onDestroy } from "svelte";

  import * as ipc from "ui/src/ipc";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import { LINK_URI_PREFIX } from "ui/src/customProtocolHandler";

  import { isContributor } from "ui/src/project";
  import {
    fetch,
    selectPeer,
    store,
    watchProjectUpdates,
  } from "ui/src/screen/project";

  import Button from "design-system/Button.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import LinkIcon from "design-system/icons/Link.svelte";
  import PenIcon from "design-system/icons/Pen.svelte";
  import ThreeDotsMenu, { MenuItem } from "design-system/ThreeDotsMenu.svelte";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  import ManagePeersModal from "./ProjectScreen/ManagePeersModal.svelte";
  import PeerSelector from "ui/App/SharedComponents/PeerSelector.svelte";
  import ProjectHeader from "./ProjectScreen/ProjectHeader.svelte";
  import Source from "./ProjectScreen/Source.svelte";
  import type * as projectRoute from "./ProjectScreen/route";
  import Patch from "./ProjectScreen/Patch.svelte";
  import PatchList from "./ProjectScreen/PatchList.svelte";

  export let urn: string;
  export let anchors: ConfirmedAnchor[];
  export let activeView: projectRoute.ProjectView = { type: "files" };
  let hoverstyle: string = "";

  const mouseenter = (): void => {
    hoverstyle = "background-color: var(--color-foreground-level-2)";
  };
  const mouseleave = (): void => {
    hoverstyle = "";
  };

  const onPeerModal = (): void => {
    modal.toggle(ManagePeersModal);
  };

  const onSelectPeer = ({ detail: peer }: { detail: User }): void => {
    selectPeer(peer);
  };

  const unwatchProjectUpdates = watchProjectUpdates(urn);
  onDestroy(unwatchProjectUpdates);

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);

  $: if ($store.status === remote.Status.Error) {
    notification.showException($store.error);
  }

  let peerSelectorExpanded: boolean = false;

  function menuItems(project: Project): MenuItem[] {
    const items: MenuItem[] = [
      {
        title: "Copy link",
        icon: LinkIcon,
        event: () => {
          ipc.copyToClipboard(`${LINK_URI_PREFIX}${project.urn}`);
          notification.show({
            type: "info",
            message: "Shareable link copied to your clipboard",
          });
        },
      },
    ];

    if (project.seed) {
      const seedUrl = new URL(project.seed);
      items.push({
        title: "View in browser",
        icon: GlobeIcon,
        disabled: false,
        tooltip: undefined,
        event: () => {
          ipc.openUrl(
            `https://app.radicle.network/seeds/${seedUrl.hostname}/${project.urn}`
          );
        },
      });
    } else {
      items.push({
        title: "View in browser",
        icon: GlobeIcon,
        disabled: true,
        tooltip: "This project isn’t on a seed yet",
        event: () => {},
      });
    }

    return items;
  }
</script>

<ScreenLayout
  dataCy="project-screen"
  contentStyle="padding: 0 0 1rem 0; max-width: 100%;">
  <div slot="header" style="display: flex">
    {#if $store.status === remote.Status.Success}
      <ProjectHeader
        urn={$store.data.project.urn}
        name={$store.data.project.metadata.name}
        description={$store.data.project.metadata.description}
        onClick={() =>
          router.push({
            type: "project",
            params: {
              urn: urn,
              activeView: { type: "files" },
            },
          })} />

      <div style="display: flex; align-self: center; margin-left: 1.5rem;">
        <div
          style="display: flex; z-index: 10; align-items: center;"
          class:button-transition={!peerSelectorExpanded}>
          <PeerSelector
            bind:expanded={peerSelectorExpanded}
            peers={$store.data.peerSelection}
            on:modal={onPeerModal}
            on:select={onSelectPeer}
            selected={$store.data.selectedPeer} />
          <Button
            ariaLabel="edit remote peers"
            dataCy="manage-remotes"
            icon={PenIcon}
            variant="outline"
            transition={false}
            on:click={onPeerModal}
            on:mouseenter={mouseenter}
            on:mouseleave={mouseleave}
            style={`margin-right: 1rem; border-top-left-radius: 0; border-bottom-left-radius: 0; padding: 0 0.5rem; ${hoverstyle}`} />
        </div>
        <ThreeDotsMenu menuItems={menuItems($store.data.project)} />
      </div>
    {/if}
  </div>
  {#if $store.status === remote.Status.Success}
    {#if activeView.type === "patch"}
      <Patch
        project={$store.data.project}
        id={activeView.id}
        peerId={activeView.peerId}
        view={activeView.view}
        patchCount={$store.data.patches.filter(
          patch => patch.status.current === "open"
        ).length} />
    {:else if activeView.type === "patches"}
      <PatchList
        project={$store.data.project}
        patches={$store.data.patches}
        filter={activeView.filter} />
    {:else}
      <Source
        {activeView}
        project={$store.data.project}
        selectedPeer={$store.data.selectedPeer}
        patches={$store.data.patches}
        {anchors}
        isContributor={isContributor($store.data.peerSelection)} />
    {/if}
  {/if}
</ScreenLayout>
