<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ObjectType } from "../../../src/source";
  import type { Tree } from "../../../src/source";

  import File from "./Tree/File.svelte";
  import Folder from "./Tree/Folder.svelte";

  export let currentPath: Readable<string>;
  export let fetchTree: (path: string) => Promise<Tree>;
  export let tree: Tree;

  const dispatch = createEventDispatcher();
  const onSelectPath = ({ detail: path }: { detail: string }): void => {
    dispatch("select", path);
  };
</script>

{#each tree.entries as entry (entry.path)}
  {#if entry.info.objectType === ObjectType.Tree}
    <Folder
      {currentPath}
      {fetchTree}
      name={entry.info.name}
      prefix={`${entry.path}/`}
      on:select={onSelectPath} />
  {:else}
    <File
      active={entry.path === $currentPath}
      dataCy={`file-${entry.path}`}
      name={entry.info.name}
      on:click={() => {
        onSelectPath({ detail: entry.path });
      }} />
  {/if}
{/each}
