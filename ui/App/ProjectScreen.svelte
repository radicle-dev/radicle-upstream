<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { ConfirmedAnchor } from "ui/src/project";

  import { onDestroy } from "svelte";

  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";

  import { isContributor } from "ui/src/project";
  import {
    fetch,
    selectPeer,
    store,
    watchProjectUpdates,
  } from "ui/src/screen/project";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  import Header from "./ProjectScreen/Header.svelte";
  import Source from "./ProjectScreen/Source.svelte";
  import type * as projectRoute from "./ProjectScreen/route";
  import Patch from "./ProjectScreen/Patch.svelte";
  import PatchList from "./ProjectScreen/PatchList.svelte";

  export let urn: string;
  export let anchors: ConfirmedAnchor[];
  export let activeView: projectRoute.ProjectView = { type: "files" };

  const unwatchProjectUpdates = watchProjectUpdates(urn);
  onDestroy(unwatchProjectUpdates);

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);

  $: if ($store.status === remote.Status.Error) {
    notification.showException($store.error);
  }
</script>

<ScreenLayout
  dataCy="project-screen"
  contentStyle="padding: 0 0 1rem 0; max-width: 100%;">
  <div slot="header">
    {#if $store.status === remote.Status.Success}
      <Header
        project={$store.data.project}
        peers={$store.data.peerSelection}
        selectedPeer={$store.data.selectedPeer}
        {selectPeer} />
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
