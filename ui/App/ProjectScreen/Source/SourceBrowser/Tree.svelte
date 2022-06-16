<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Readable } from "svelte/store";

  import { ObjectType } from "ui/src/source";
  import type { SelectedPath, Tree, SelectedRevision } from "ui/src/source";

  import File from "./Tree/File.svelte";
  import Folder from "./Tree/Folder.svelte";

  export let projectUrn: string;
  export let peerId: string;
  export let selectedRevision: SelectedRevision;
  export let tree: Readable<Tree>;
  export let selectedPath: SelectedPath;
  export let selectPath: (path: string) => void;
</script>

{#each $tree.entries as entry (entry.path)}
  {#if entry.info.objectType === ObjectType.Tree}
    <Folder
      name={entry.info.name}
      path={entry.path}
      {projectUrn}
      {peerId}
      {selectedRevision}
      {selectPath}
      {selectedPath} />
  {:else}
    <File
      active={entry.path === selectedPath.selected}
      dataCy={`file-${entry.path}`}
      loading={entry.path === selectedPath.selected &&
        selectedPath.request !== null}
      name={entry.info.name}
      on:click={() => {
        selectPath(entry.path);
      }} />
  {/if}
{/each}
