<script lang="typescript">
  import * as router from "svelte-spa-router";

  import * as path from "../../../src/path";
  import type { Project } from "../../../src/project";
  import type { MergeRequest } from "../../../src/project/mergeRequest";
  import {
    EmptyState,
    List,
    SegmentedControl,
  } from "../../../DesignSystem/Component";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];
  export let project: Project;

  const defaultBranch = project.metadata.defaultBranch;

  const selectMergeRequest = ({
    detail: mergeRequest,
  }: {
    detail: MergeRequest;
  }) => {
    router.push(
      path.projectSourceMergeRequest(
        project.urn,
        mergeRequest,
        project.metadata.defaultBranch
      )
    );
  };

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
      on:select={option => {
        currentFilter = option.detail;
      }} />
  </div>
  {#if filteredMergeRequests.length > 0}
    <List
      dataCy="merge-request-list"
      items={filteredMergeRequests}
      on:select={selectMergeRequest}
      let:item={mergeRequest}
      style="margin: 0 auto; overflow: visible;">
      <div class="list-item" data-cy={`project-list-entry-${mergeRequest}`}>
        <MergeRequestCard {defaultBranch} {mergeRequest} />
      </div>
    </List>
  {:else}
    <EmptyState
      emoji="👯‍♀️"
      text="There’s nothing to show here at the moment. If you’re looking for a peer’s Merge Request, be sure to add that peer’s Device ID as a remote." />
  {/if}
</div>
