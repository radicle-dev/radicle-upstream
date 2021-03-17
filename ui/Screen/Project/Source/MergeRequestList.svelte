<script lang="typescript">
  import type { Branch, MergeRequest } from "../../../src/source";

  import { List, SegmentedControl } from "../../../DesignSystem/Component";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];
  export let defaultBranch: Branch;

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
</div>
