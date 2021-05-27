<script lang="typescript">
  import * as router from "svelte-spa-router";

  import * as path from "ui/src/path";
  import type { Project } from "ui/src/project";
  import type { Patch } from "ui/src/project/patch";
  import {
    EmptyState,
    List,
    SegmentedControl,
  } from "ui/DesignSystem/Component";
  import PatchCard from "./PatchCard.svelte";

  export let patches: Patch[];
  export let project: Project;

  const defaultBranch = project.metadata.defaultBranch;

  const selectPatch = ({ detail: patch }: { detail: Patch }) => {
    router.push(path.projectSourcePatch(project.urn, patch.peerId, patch.id));
  };

  const filterOptions = [
    {
      title: "Open",
      value: path.PatchFilter.Open,
    },
    {
      title: "Closed",
      value: path.PatchFilter.Closed,
    },
    {
      title: "All",
      value: path.PatchFilter.All,
    },
  ];

  const query = path.projectSourcePatchesQuery();
  $: filter = $query?.filter;
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
  <div class="filters">
    <SegmentedControl
      active={filter}
      options={filterOptions}
      on:select={option => {
        router.push(
          path.projectSourcePatchesFilter(project.urn, option.detail)
        );
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
      text="There’s nothing to show here at the moment. If you’re looking
      for a peer’s Patch, be sure to add that peer’s Device ID as a remote." />
  {/if}
</div>
