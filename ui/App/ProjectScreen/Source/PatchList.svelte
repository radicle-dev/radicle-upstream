<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as router from "ui/src/router";
  import { unreachable } from "ui/src/unreachable";

  import type { Project } from "ui/src/project";
  import type { Patch } from "ui/src/project/patch";
  import { List, SegmentedControl } from "ui/DesignSystem";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import PatchCard from "./PatchCard.svelte";

  export let patches: Patch[];
  export let project: Project;
  export let filter: "open" | "closed" | "all";

  const defaultBranch = project.metadata.defaultBranch;

  const selectPatch = ({ detail: patch }: { detail: Patch }) => {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: {
          type: "patch",
          peerId: patch.peerId,
          id: patch.id,
        },
      },
    });
  };

  const filterOptions = [
    {
      title: "Open",
      value: "open",
    },
    {
      title: "Closed",
      value: "closed",
    },
    {
      title: "All",
      value: "all",
    },
  ];

  let filteredPatches: Patch[];
  $: {
    switch (filter) {
      case "open":
        filteredPatches = patches.filter(patch => !patch.merged);
        break;
      case "closed":
        filteredPatches = patches.filter(patch => patch.merged);
        break;
      case "all":
        filteredPatches = patches;
        break;
      default:
        unreachable(filter);
        break;
    }
  }
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 0 auto;
    min-width: var(--content-min-width);
  }
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }
  .filters {
    margin: 0.5rem 0 1.5rem;
    padding: 0 var(--content-padding);
  }
</style>

<div class="container">
  <div class="filters" data-cy="patch-filter-tabs">
    <SegmentedControl
      active={filter}
      options={filterOptions}
      on:select={option => {
        router.push({
          type: "project",
          params: {
            urn: project.urn,
            activeView: { type: "patches", filter: option.detail },
          },
        });
      }} />
  </div>
  {#if filteredPatches.length > 0}
    <List
      dataCy="patch-list"
      items={filteredPatches}
      on:select={selectPatch}
      let:item={patch}
      style="margin: 0 auto; overflow: visible;">
      <div class="list-item" data-cy={`project-list-entry-${patch}`}>
        <PatchCard {defaultBranch} {patch} />
      </div>
    </List>
  {:else}
    <EmptyState
      emoji="🎏"
      text="There are no patches to show at the moment. If you’re looking
      for someone’s patch, be sure to add their Device ID as a remote using the dropdown above." />
  {/if}
</div>
