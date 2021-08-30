<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ViewKind } from "ui/src/screen/project/source";
  import type { Code, View } from "ui/src/screen/project/source";
  import type { Tree } from "ui/src/source";

  import EmptyState from "ui/DesignSystem/EmptyState.svelte";

  import Blob from "./FileView/Blob.svelte";
  import Root from "./FileView/Root.svelte";

  export let code: Readable<Code>;
  export let tree: Readable<Tree>;

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
    <Root
      emptyRepo={$tree.entries.length === 0}
      commit={$code.lastCommit}
      {view}
      on:select={onSelectCommit} />
  {:else if view.kind === ViewKind.Error}
    <EmptyState
      emoji="👀"
      headerText={view.error.message}
      on:primaryAction={onSelectRoot}
      primaryActionText="Back to files"
      style="height: 320px;"
      text="This file doesn’t exist on this branch." />
  {/if}
</div>
