<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ObjectType } from "ui/src/source";
  import type { SelectedPath, SelectedRevision, Tree } from "ui/src/source";

  import File from "./Tree/File.svelte";
  import Folder from "./Tree/Folder.svelte";

  export let fetchTree: (path: string) => Promise<Tree>;
  export let selectedPath: Readable<SelectedPath>;
  export let selectedRevision: SelectedRevision;
  export let tree: Readable<Tree>;

  const dispatch = createEventDispatcher();
  const onSelectPath = ({ detail: path }: { detail: string }): void => {
    dispatch("select", path);
  };
</script>

{#each $tree.entries as entry (entry.path)}
  {#if entry.info.objectType === ObjectType.Tree}
    <Folder
      {fetchTree}
      name={entry.info.name}
      prefix={`${entry.path}/`}
      on:select={onSelectPath}
      {selectedPath}
      {selectedRevision} />
  {:else}
    <File
      active={entry.path === $selectedPath.selected}
      dataCy={`file-${entry.path}`}
      loading={entry.path === $selectedPath.selected &&
        $selectedPath.request !== null}
      name={entry.info.name}
      on:click={() => {
        onSelectPath({ detail: entry.path });
      }} />
  {/if}
{/each}
