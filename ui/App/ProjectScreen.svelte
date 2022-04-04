<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { onDestroy } from "svelte";

  import * as localPeer from "ui/src/localPeer";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import type { User, ConfirmedAnchor } from "ui/src/project";
  import { isContributor } from "ui/src/project";
  import {
    fetch,
    selectPeer,
    refreshPeers,
    store,
  } from "ui/src/screen/project";

  import PenIcon from "design-system/icons/Pen.svelte";

  import Button from "design-system/Button.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  import ManagePeersModal from "./ProjectScreen/ManagePeersModal.svelte";
  import PeerSelector from "ui/App/SharedComponents/PeerSelector.svelte";
  import ProjectHeader from "./ProjectScreen/ProjectHeader.svelte";
  import Source from "./ProjectScreen/Source.svelte";
  import type * as projectRoute from "./ProjectScreen/route";

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

  const unsubscribeFromProjectEvents = localPeer.projectEvents.onValue(
    event => {
      if (event.urn === urn) {
        refreshPeers();
      }
    }
  );
  onDestroy(unsubscribeFromProjectEvents);

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);

  $: if ($store.status === remote.Status.Error) {
    notification.showException($store.error);
  }

  let peerSelectorExpanded: boolean = false;
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
        stats={$store.data.project.stats}
        latestAnchorTimestamp={anchors.slice(-1)[0]?.timestamp}
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
            dataCy="manage-remotes"
            icon={PenIcon}
            variant="outline"
            transition={false}
            on:click={onPeerModal}
            on:mouseenter={mouseenter}
            on:mouseleave={mouseleave}
            style={`border-top-left-radius: 0; border-bottom-left-radius: 0; padding: 0 0.5rem; ${hoverstyle}`} />
        </div>
      </div>
    {/if}
  </div>
  {#if $store.status === remote.Status.Success}
    <Source
      {activeView}
      project={$store.data.project}
      selectedPeer={$store.data.selectedPeer}
      {anchors}
      isContributor={isContributor($store.data.peerSelection)} />
  {/if}
</ScreenLayout>
