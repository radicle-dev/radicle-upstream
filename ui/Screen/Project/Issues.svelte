<script>
  import * as path from "../../src/path.ts";
  import { push } from "svelte-spa-router";
  import IssuesCard from "./IssuesCard.svelte";
  import { SegmentedControl } from "../../DesignSystem/Component";

  const filterOptions = [
    {
      title: "Open",
      value: 0,
    },
    {
      title: "Closed",
      value: 1,
    },
    {
      title: "All",
      value: 2,
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
      active={0}
      options={filterOptions}
      on:select={(option) => updateFilter(option.detail.title)} />
  </div>
  <ul>
    {#each filteredIssues as issue}
      <li
        on:click={() => {
          push(path.projectIssue(params.id));
        }}>
        <IssuesCard
          open={issue.open}
          title={issue.title}
          author={issue.author}
          replies={issue.replies}
          created={issue.created_at}
          updated={issue.updated_at}
          closed={issue.closed_at} />
      </li>
    {/each}
  </ul>
</div>
