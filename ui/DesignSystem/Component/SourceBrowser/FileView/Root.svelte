<script lang="typescript">
  import type { CommitHeader } from "../../../../src/source";
  import type { Root } from "../../../../src/screen/project/source";

  import EmptyState from "../../EmptyState.svelte";
  import Readme from "../Readme.svelte";

  import CommitTeaser from "../CommitTeaser.svelte";

  export let commit: CommitHeader;
  export let view: Root;
  export let emptyRepo: boolean;
</script>

<style>
  .commit-header {
    height: 2.5rem;
    margin-bottom: 1rem;
  }
</style>

<div class="commit-header">
  <CommitTeaser {commit} on:select style="height: 100%" />
</div>

{#if view.readme}
  <Readme content={view.readme.content} path={view.readme.path} />
{:else if emptyRepo}
  EMPTY REPO
{:else}
  <EmptyState
    text="This project doesn't have a README yet."
    emoji="👀"
    style="height: 320px;" />
{/if}
