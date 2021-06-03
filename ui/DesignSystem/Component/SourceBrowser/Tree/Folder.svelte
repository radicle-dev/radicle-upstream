<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import { ObjectType } from "../../../../src/source";
  import type {
    SelectedPath,
    SelectedRevision,
    Tree,
  } from "../../../../src/source";

  import { Icon } from "../../../Primitive";

  import File from "./File.svelte";

  export let fetchTree: (path: string) => Promise<Tree>;
  export let name: string;
  export let prefix: string;
  export let selectedPath: Readable<SelectedPath>;
  export let selectedRevision: SelectedRevision;

  let expanded = false;

  const dispatch = createEventDispatcher();
  const onSelectPath = ({ detail: path }: { detail: string }) => {
    dispatch("select", path);
  };
  const toggle = () => {
    expanded = !expanded;
  };

  let current: Promise<Tree>;

  $: if (selectedRevision.request === null) {
    current = fetchTree(prefix);
  }
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
    border-radius: 0.5rem;
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
  <span style="height: 1.5rem">
    <svelte:component
      this={expanded ? Icon.ChevronDown : Icon.ChevronRight}
      dataCy={`expand-${name}`} />
  </span>
  <span class="folder-name">{name}</span>
</div>

<div class="container">
  {#await current then tree}
    {#if expanded}
      {#each tree.entries as entry (entry.path)}
        {#if entry.info.objectType === ObjectType.Tree}
          <svelte:self
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
    {/if}
  {/await}
</div>
