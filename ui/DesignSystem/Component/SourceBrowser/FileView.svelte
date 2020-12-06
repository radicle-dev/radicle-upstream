<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ViewKind } from "../../../src/screen/project/source";
  import type { Code, View } from "../../../src/screen/project/source";

  import EmptyState from "../../../DesignSystem/Component/EmptyState.svelte";

  import Blob from "./FileView/Blob.svelte";
  import Root from "./FileView/Root.svelte";

  export let code: Readable<Code>;

  const dispatch = createEventDispatcher();
  const onSelectCommit = ({ detail: sha1 }: { detail: string }) =>
    dispatch("commit", sha1);
  const onSelectRoot = () => dispatch("root");

  let view: View;
  $: view = $code.view;
</script>

<div data-cy="file-view">
  {#if view.kind === ViewKind.Blob}
    <Blob {view} on:root={onSelectRoot} on:select={onSelectCommit} />
  {:else if view.kind === ViewKind.Root}
    <Root commit={$code.lastCommit} {view} on:select={onSelectCommit} />
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
