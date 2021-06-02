<script lang="typescript">
  import type { CommitStats } from "../../../src/source";
  import type { Diff } from "../../../src/source/diff";

  import IconFile from "../../Primitive/Icon/File.svelte";

  import FileDiff from "./FileDiff.svelte";

  export let diff: Diff;
  export let stats: CommitStats;
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

  .file-header {
    height: 3rem;
    display: flex;
    align-items: center;
    background: none;
    border-bottom: 1px solid var(--color-foreground-level-3);
    border-radius: 0;
    padding: 0.75rem;
  }

  .file-header:last-child {
    border-bottom: none;
    margin-bottom: 1rem;
  }

  .file-header .diff-type {
    margin-left: 1rem;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
  }

  .file-header .diff-type.created {
    color: var(--color-positive);
    background-color: var(--color-positive-level-1);
  }

  .file-header .diff-type.deleted {
    color: var(--color-negative);
    background-color: var(--color-negative-level-1);
  }
</style>

<div class="changeset-summary">
  {#if diff.modified.length > 0}
    <span class="typo-semi-bold"> {diff.modified.length} file(s) changed </span>
    with
    <span class="additions typo-semi-bold"> {stats.additions} additions </span>
    and
    <span class="deletions typo-semi-bold"> {stats.deletions} deletions </span>
  {/if}
</div>
<div>
  {#each diff.created as path (path)}
    <header class="file-header">
      <IconFile style="margin-right: 8px;" />
      <p class="typo-text-bold">{path}</p>
      <span class="diff-type created">created</span>
    </header>
  {/each}
  {#each diff.deleted as path (path)}
    <header class="file-header">
      <IconFile style="margin-right: 8px;" />
      <p class="typo-text-bold">{path}</p>
      <span class="diff-type deleted">deleted</span>
    </header>
  {/each}
</div>

{#each diff.modified as file}
  <FileDiff {file} />
{/each}
