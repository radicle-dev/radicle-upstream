<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { ViewKind, type Code } from "ui/src/screen/project/source";
  import type { Tree } from "ui/src/source";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";

  import FileSource from "./FileSource.svelte";
  import Root from "./FileView/Root.svelte";

  export let code: Code;
  export let tree: Tree;

  const dispatch = createEventDispatcher();
  const onSelectCommit = ({ detail: sha1 }: { detail: string }): void => {
    dispatch("commit", sha1);
  };
  const onSelectRoot = (): void => {
    dispatch("root");
  };
</script>

<div data-cy="file-view">
  {#if code.view.kind === ViewKind.Blob}
    <FileSource
      blob={code.view.blob}
      commit={code.view.blob.info.lastCommit}
      on:root={onSelectRoot}
      on:select={onSelectCommit} />
  {:else if code.view.kind === ViewKind.Root}
    <Root
      emptyRepo={tree.entries.length === 0}
      commit={code.lastCommit}
      view={code.view}
      on:select={onSelectCommit} />
  {:else if code.view.kind === ViewKind.Error}
    <EmptyState
      emoji="👀"
      headerText={code.view.error.message}
      on:primaryAction={onSelectRoot}
      primaryActionText="Back to files"
      style="height: 320px;"
      text="This file doesn’t exist on this branch." />
  {/if}
</div>
