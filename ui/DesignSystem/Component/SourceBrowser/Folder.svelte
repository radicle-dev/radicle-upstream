<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import type { PeerId } from "../../../src/identity";
  import { tree, objectPath, ObjectType } from "../../../src/source";
  import type { Revision } from "../../../src/source";
  import type { Urn } from "../../../src/urn";

  import { Icon } from "../../Primitive";
  import { Remote } from "../../Component";

  import File from "./File.svelte";

  export let currentPath: Readable<string>;
  export let name: string = "";
  export let peerId: PeerId;
  export let prefix: string;
  export let projectUrn: Urn;
  export let revision: Revision;

  let expanded = false;

  const dispatch = createEventDispatcher();
  const onSelectPath = ({ detail: path }: { detail: string }) => {
    dispatch("select", path);
  };
  const toggle = () => {
    expanded = !expanded;
  };

  $: store = tree(projectUrn, peerId, revision, prefix);
  $: active = prefix === $objectPath;
</script>

<style>
  .folder {
    display: flex;
    cursor: pointer;
    padding: 0.25rem 0.25rem 0.25rem 0.25rem;
    margin: 0.25rem 0;
    color: var(--color-foreground-level-6);
    user-select: none;
    line-height: 1.5rem;
    white-space: nowrap;
  }
  .folder:hover {
    background-color: var(--color-foreground-level-1);
    border-radius: 0.25rem;
  }

  .folder-name {
    margin-left: 0.25rem;
  }

  .container {
    padding-left: 0.5rem;
    margin: 0;
  }
</style>

<div class="folder" on:click={toggle}>
  <span class:active style="height: 1.5rem">
    <svelte:component
      this={expanded ? Icon.ChevronDown : Icon.ChevronRight}
      dataCy={`expand-${name}`} />
  </span>
  <span class="folder-name">{name}</span>
</div>

<div class="container">
  <Remote {store} let:data={tree}>
    {#if expanded}
      {#each tree.entries as entry}
        {#if entry.info.objectType === ObjectType.Tree}
          <svelte:self
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
            name={entry.info.name}
            on:click={() => {
              onSelectPath({ detail: entry.path });
            }} />
        {/if}
      {/each}
    {/if}
  </Remote>
</div>
