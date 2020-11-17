<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ViewKind } from "../../../src/screen/project/source";
  import type { Code, View } from "../../../src/screen/project/source";

  import EmptyState from "../../../DesignSystem/Component/EmptyState.svelte";

  import CommitTeaser from "./CommitTeaser.svelte";

  import Blob from "./FileView/Blob.svelte";
  import Root from "./FileView/Root.svelte";

  export let code: Readable<Code>;
  export let rootName: string;

  const dispatch = createEventDispatcher();
  const onSelectCommit = ({ detail: sha1 }: { detail: string }) =>
    dispatch("commit", sha1);
  const onSelectRoot = () => dispatch("root");

  let view: View;
  $: view = $code.view;
</script>

<style>
  .commit-header {
    height: 2.5rem;
    margin-bottom: 1rem;
  }
</style>

<div data-cy="file-view">
  <div class="commit-header">
    <CommitTeaser
      commit={$code.lastCommit}
      on:select={onSelectCommit}
      style="height: 100%" />
  </div>

  {#if view.kind === ViewKind.Blob}
    <Blob {rootName} {view} on:root={onSelectRoot} />
  {:else if view.kind === ViewKind.Root}
    <Root {view} />
  {:else if view.kind === ViewKind.Error}
    <EmptyState
      emoji="👀"
      headerText={view.error.message}
      on:primaryAction={onSelectRoot}
      primaryActionText="Back to files"
      style="height: 320px;"
      text="This file doesn't exist on this branch." />
  {/if}
</div>
