<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Diff, CommitStats } from "proxy-client/commit";

  import FileDiff from "./FileDiff.svelte";

  export let diff: Diff;
  export let stats: CommitStats;

  const changedFileCount: number =
    diff.modified.length + diff.created.length + diff.deleted.length;

  function pluralize(word: string, count: number) {
    if (count > 1) {
      return `${count} ${word}s`;
    } else {
      return `${count} ${word}`;
    }
  }
</script>

<style>
  .changeset-summary {
    margin-top: 2rem;
    margin-bottom: 1.5rem;
    margin-left: 1.5rem;
  }

  .changeset-summary .additions {
    color: var(--color-positive);
  }

  .changeset-summary .deletions {
    color: var(--color-negative);
  }
</style>

<div class="changeset-summary">
  {#if changedFileCount > 0}
    <span class="typo-semi-bold">
      {pluralize("file", changedFileCount)}
      changed
    </span>
    with
    <span class="additions typo-semi-bold">
      {pluralize("addition", stats.additions)}
    </span>
    and
    <span class="deletions typo-semi-bold">
      {pluralize("deletion", stats.deletions)}
    </span>
  {/if}
</div>
<div>
  {#each diff.created as created (created)}
    <FileDiff file={created} label="created" />
  {/each}
  {#each diff.deleted as deleted (deleted)}
    <FileDiff file={deleted} label="deleted" />
  {/each}
</div>

{#each diff.modified as file}
  <FileDiff {file} />
{/each}
