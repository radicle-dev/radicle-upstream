<script>
  import { TREE } from "../../../../native/types.js";
  import { currentPath, currentRevision, tree } from "../../../src/source.ts";
  import * as remote from "../../../src/remote.ts";

  import { Icon } from "../../Primitive";
  import File from "./File.svelte";

  export let prefix = ""; // start sidebar tree from repository root
  export let name = null;
  export let projectId = null;

  export let expanded = false;
  export let toplevel = false;

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
    padding: 4px 4px 4px 4px;
    margin: 4px 0;
    color: var(--color-foreground-level-6);
    user-select: none;
    line-height: 1.5rem;
  }
  .folder:hover {
    background-color: var(--color-foreground-level-1);
    border-radius: 4px;
  }

  .folder-name {
    margin-left: 0.25rem;
  }

  .expanded :global(svg) {
    transform: rotate(90deg);
  }

  .container {
    padding-left: 0.5rem;
    margin: 0;
  }
  .container.toplevel {
    padding-left: 0;
  }
</style>

{#if $sourceTree.status === remote.Status.Success}
  {#if !toplevel}
    <div class="folder" on:click={toggle}>
      <span class:expanded class:active style="height: 24px">
        <Icon.CarretBig dataCy={`expand-${name}`} />
      </span>
      <span class="folder-name">{name}</span>
    </div>
  {/if}

  <div class="container" class:toplevel>
    {#if expanded || toplevel}
      {#each $sourceTree.data.entries as entry}
        {#if entry.info.objectType === TREE}
          <svelte:self
            {projectId}
            name={entry.info.name}
            prefix={`${entry.path}/`} />
        {:else}
          <File {projectId} filePath={entry.path} name={entry.info.name} />
        {/if}
      {/each}
    {/if}
  </div>
{/if}
