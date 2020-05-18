<script>
  import * as path from "../../src/path.ts";
  import { push } from "svelte-spa-router";
  import IssueCard from "./Issues/IssueCard.svelte";
  import { SegmentedControl } from "../../DesignSystem/Component";

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

  export let params = null;
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
      ? issues.filter((issue) => issue.open)
      : issues.filter((issue) => !issue.open);
</script>

<style>
  .container {
    max-width: 71.25rem;
    margin: 0 auto;
    padding: 32px 0;
  }
  ul > li {
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  ul > li:last-child {
    border-bottom: 0;
  }

  .filters {
    margin-bottom: 24px;
  }
</style>

<div class="container">
  <div class="filters">
    <SegmentedControl
      active={'Open'}
      options={filterOptions}
      on:select={(option) => updateFilter(option.detail)} />
  </div>
  <ul>
    {#each filteredIssues as issue}
      <li
        on:click={() => {
          push(path.projectIssue(params.id));
        }}>
        <IssueCard {issue} />
      </li>
    {/each}
  </ul>
</div>
