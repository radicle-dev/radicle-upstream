<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project } from "ui/src/project";

  import * as router from "ui/src/router";
  import * as Patch from "ui/src/project/patch";
  import { unreachable } from "ui/src/unreachable";

  import Button from "design-system/Button.svelte";
  import List from "design-system/List.svelte";
  import SegmentedControl from "design-system/SegmentedControl.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";
  import MergeIcon from "design-system/icons/Merge.svelte";
  import CrossIcon from "design-system/icons/Cross.svelte";

  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import { makeTabs } from "./tabs";
  import PatchCard from "./PatchCard.svelte";

  export let patches: Patch.Patch[];
  export let project: Project;
  export let filter: "open" | "closed" | "merged" | "all";

  const selectPatch = ({ detail: patch }: { detail: Patch.Patch }): void => {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: {
          type: "patch",
          peerId: patch.peerId,
          id: patch.id,
          view: "commits",
        },
      },
    });
  };

  $: openPatches = patches.filter(patch => patch.status.current === "open");
  $: mergedPatches = patches.filter(patch => patch.status.current === "merged");
  $: closedPatches = patches.filter(patch => patch.status.current === "closed");

  $: filterOptions = [
    {
      title: "Open",
      value: "open",
      counter: openPatches.length,
      icon: RevisionIcon,
    },
    {
      title: "Merged",
      value: "merged",
      counter: mergedPatches.length,
      icon: MergeIcon,
    },
    {
      title: "Closed",
      value: "closed",
      counter: closedPatches.length,
      icon: CrossIcon,
    },
    {
      title: "All",
      value: "all",
      counter: patches.length,
    },
  ];

  let filteredPatches: Patch.Patch[];
  $: {
    switch (filter) {
      case "open":
        filteredPatches = openPatches;
        break;
      case "closed":
        filteredPatches = closedPatches;
        break;
      case "merged":
        filteredPatches = mergedPatches;
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
  .filters {
    margin: 0.5rem 0 1.5rem;
    padding: 0 var(--content-padding);
  }
</style>

<ActionBar>
  <TabBar
    tabs={makeTabs({
      projectUrn: project.urn,
      activeViewType: "patches",
      patchCount: patches.filter(patch => patch.status.current === "open")
        .length,
      commitCount: project.stats.commits,
    })} />
  <div style="margin-left: auto" />
  <CommandModal
    let:prop={toggleDropdown}
    command={"upstream patch create"}
    description="To create a patch in your working copy, check out the branch that contains the changes and run the following command:">
    <Button
      variant="transparent"
      icon={RevisionIcon}
      on:click={toggleDropdown}
      dataCy="patch-modal-toggle">Create patch</Button>
  </CommandModal>
</ActionBar>

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
      style="margin: 2rem; overflow: visible;">
      <PatchCard {patch} projectId={project.urn} />
    </List>
  {:else}
    <EmptyState
      style="height: calc(100vh - var(--bigheader-height) - var(--topbar-height) - 38px);"
      emoji="🎏"
      text="There are no patches to show at the moment. If you’re looking
      for someone’s patch, be sure to add their Peer ID as a remote using the dropdown above." />
  {/if}
</div>
