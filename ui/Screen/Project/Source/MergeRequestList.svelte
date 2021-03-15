<script lang="typescript">
  import type { MergeRequest } from "../../../src/source";

  import { List, SegmentedControl } from "../../../DesignSystem/Component";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];

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
  // $: filteredIssues =
  //   currentFilter === "All"
  //     ? issues
  //     : currentFilter === "Open"
  //     ? issues.filter(issue => issue.open)
  //     : issues.filter(issue => !issue.open);
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
    margin-bottom: var(--content-padding);
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
    items={mergeRequests}
    on:select
    let:item={mergeRequest}
    style="margin: 0 auto; overflow: visible;">
    <div class="list-item" data-cy={`project-list-entry-${mergeRequest}`}>
      <MergeRequestCard {mergeRequest} />
    </div>
  </List>
</div>
