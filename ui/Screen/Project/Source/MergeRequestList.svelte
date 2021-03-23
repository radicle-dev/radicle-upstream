<script lang="typescript">
  import type { Branch, MergeRequest } from "../../../src/source";

  import {
    Copyable,
    EmptyState,
    List,
    SegmentedControl,
  } from "../../../DesignSystem/Component";
  import { Button } from "../../../DesignSystem/Primitive";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];
  export let defaultBranch: Branch;

  let copyable: Copyable;
  const instructions = `git tag --annotate merge-request/tag-name
git push --tags rad`;

  const filterOptions = [
    {
      title: "Open",
      value: "Open",
    },
    {
      title: "Closed",
      value: "Closed",
    },
    {
      title: "All",
      value: "All",
    },
  ];

  let currentFilter = "Open";
  function updateFilter(newFilter) {
    currentFilter = newFilter;
  }
  $: filteredMergeRequests =
    currentFilter === "All"
      ? mergeRequests
      : currentFilter === "Open"
      ? mergeRequests.filter(mergeRequest => !mergeRequest.merged)
      : mergeRequests.filter(mergeRequest => mergeRequest.merged);
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
      active={'Open'}
      options={filterOptions}
      on:select={option => updateFilter(option.detail)} />
  </div>
  {#if filteredMergeRequests.length > 0}
    <List
      dataCy="merge-request-list"
      items={filteredMergeRequests}
      on:select
      let:item={mergeRequest}
      style="margin: 0 auto; overflow: visible;">
      <div class="list-item" data-cy={`project-list-entry-${mergeRequest}`}>
        <MergeRequestCard {defaultBranch} {mergeRequest} />
      </div>
    </List>
  {:else}
    <EmptyState
      emoji="👯‍♀️"
      text="There’s nothing here yet, get started by opening your first merge request.">
      <Copyable bind:this={copyable} showIcon={false}>
        <pre
          class="typo-text-small-mono"
          style="text-align: left; color: var(--color-foreground-level-6); overflow-x: scroll; padding: .5rem .5rem .5rem .25rem">
          {instructions}
        </pre>
      </Copyable>
      <Button
        variant="primary"
        style="display: block; margin: 1rem auto 0;"
        on:click={() => copyable.copy()}>
        Copy
      </Button>
    </EmptyState>
  {/if}
</div>
