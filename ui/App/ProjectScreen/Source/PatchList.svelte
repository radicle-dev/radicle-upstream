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
  import Hoverable from "design-system/Hoverable.svelte";
  import LinkIcon from "design-system/icons/Link.svelte";
  import List from "design-system/List.svelte";
  import SegmentedControl from "design-system/SegmentedControl.svelte";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import PatchCard from "./PatchCard.svelte";

  export let patches: Patch.Patch[];
  export let project: Project;
  export let filter: "open" | "closed" | "all";

  const selectPatch = ({ detail: patch }: { detail: Patch.Patch }): void => {
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

  let filteredPatches: Patch.Patch[];
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
      style="margin: 2rem; overflow: visible;">
      <Hoverable let:hovering={hover} style="flex: 1;">
        <div class="list-item">
          <PatchCard {patch}>
            {#if hover}
              <Button
                dataCy="copy-url"
                variant="vanilla"
                icon={LinkIcon}
                on:click={() => {
                  Patch.copyPatchUrlToClipboard(project.urn, patch);
                }}>Copy link</Button>
            {/if}
          </PatchCard>
        </div>
      </Hoverable>
    </List>
  {:else}
    <EmptyState
      style="height: calc(100vh - var(--bigheader-height) - var(--topbar-height) - 38px);"
      emoji="🎏"
      text="There are no patches to show at the moment. If you’re looking
      for someone’s patch, be sure to add their Peer ID as a remote using the dropdown above." />
  {/if}
</div>
