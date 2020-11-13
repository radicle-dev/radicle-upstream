<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import type { PeerId } from "../../../src/identity";
  import { ObjectType } from "../../../src/source";
  import type { Branch, Tag, Tree } from "../../../src/source";
  import type { Urn } from "../../../src/urn";

  import File from "./File.svelte";
  import Folder from "./Folder.svelte";

  export let currentPath: Readable<string>;
  export let peerId: PeerId;
  export let projectUrn: Urn;
  export let revision: Branch | Tag;
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
      name={entry.info.name}
      {peerId}
      prefix={`${entry.path}/`}
      {projectUrn}
      {revision}
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
