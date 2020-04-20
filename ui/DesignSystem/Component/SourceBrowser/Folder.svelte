<script>
  import { getContext } from "svelte";
  import { link } from "svelte-spa-router";

  import { currentPath, currentRevision, tree } from "../../../src/source.ts";
  import * as path from "../../../lib/path.js";
  import { TREE } from "../../../../native/types.js";

  import { Icon } from "../../Primitive";
  import File from "./File.svelte";

  export let prefix = ""; // start sidebar tree from repository root
  export let name = null;

  export let expanded = false;
  export let firstEntry = true;

  const projectId = getContext("projectId");

  $: sourceTree = tree(projectId, $currentRevision, prefix);

  const toggle = () => {
    expanded = !expanded;
  };

  $: active = prefix === $currentPath;
</script>

<style>
  .folder {
    display: flex;
    cursor: pointer;
    margin: 0 4px 12px 0;
    color: var(--color-foreground-level-6);
    user-select: none;
  }

  .expanded :global(svg) {
    transform: rotate(90deg);
  }

  .folder :global(svg:hover) {
    background-color: var(--color-foreground-level-2);
    border-radius: 2px;
  }

  .container {
    margin: 0 0 0 8px;
  }

  a {
    display: flex;
  }

  .active a {
    color: var(--color-secondary);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-secondary);
  }
</style>

{#if $sourceTree.status === 'NOT_ASKED'}
  <p>Not asked...</p>
{:else if $sourceTree.status === 'LOADING'}
  <p>Loading...</p>
{:else if $sourceTree.status === 'SUCCESS'}
  <div class="container">
    {#if firstEntry}
      <div class="folder" class:active>
        <Icon.Folder dataCy={`expand-${name}`} />
        <a
          href={path.projectSource(projectId, $currentRevision, TREE, prefix)}
          use:link>
          {name}
        </a>
      </div>
    {:else}
      <div class="folder" class:expanded class:active>
        <Icon.CarretBig dataCy={`expand-${name}`} on:click={toggle} />
        <a
          href={path.projectSource(projectId, $currentRevision, TREE, prefix)}
          use:link>
          {name}
        </a>
      </div>
    {/if}

    {#if expanded || firstEntry}
      {#each $sourceTree.data.entries as entry}
        {#if entry.info.objectType === TREE}
          <svelte:self
            prefix={`${entry.path}/`}
            name={entry.info.name}
            firstEntry={false} />
        {:else}
          <File name={entry.info.name} filePath={entry.path} />
        {/if}
      {/each}
    {/if}
  </div>
{:else if $sourceTree.status === 'ERROR'}
  <p>{`error: ${$sourceTree.error.message}`}</p>
{/if}
