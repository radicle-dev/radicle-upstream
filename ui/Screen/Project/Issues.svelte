<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script>
  import IssueCard from "./Issues/IssueCard.svelte";
  import { EmptyState, List, SegmentedControl } from "ui/DesignSystem";

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

  const issues = [
    {
      hash: "blka",
      open: true,
      title: "Split server into CentralChain, and ChainApi",
      author: "julien",
      replies: 12,
      created_at: "12 days",
      updated_at: "1 day",
      closed_at: null,
    },
    {
      hash: "blkhjghfdga",
      open: true,
      title: "Feature request: Call JSON.stringify before displaying objects",
      author: "xla",
      replies: 0,
      created_at: "20 days",
      updated_at: "18 days",
      closed_at: null,
    },
    {
      hash: "blkfsfsa",
      open: false,
      title: "Return permissions via the session endpoint",
      author: "rudolfs",
      replies: 2,
      created_at: "1 month",
      updated_at: "27 days",
      closed_at: "27 days",
    },
  ];
  let currentFilter = "Open";
  function updateFilter(newFilter) {
    currentFilter = newFilter;
  }
  $: filteredIssues =
    currentFilter === "All"
      ? issues
      : currentFilter === "Open"
      ? issues.filter(issue => issue.open)
      : issues.filter(issue => !issue.open);
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 0 auto;
    min-width: var(--content-min-width);
  }
  .filters {
    margin-bottom: var(--content-padding);
    padding: 0 var(--content-padding);
  }
</style>

<div class="container">
  <div class="filters">
    <SegmentedControl
      active={"Open"}
      options={filterOptions}
      on:select={option => updateFilter(option.detail)} />
  </div>
  {#if issues.length > 0}
    <List dataCy="issue-list" items={filteredIssues} let:item={issue}>
      <IssueCard {issue} />
    </List>
  {:else}
    <EmptyState
      emoji="🔭"
      text="There’s nothing here yet, get started by creating your first issue."
      primaryActionText="Open a new issue" />
  {/if}
</div>
