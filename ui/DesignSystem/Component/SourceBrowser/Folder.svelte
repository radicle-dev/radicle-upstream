<script>
  import {
    currentPath,
    currentRevision,
    tree,
    updateParams,
    ObjectType,
  } from "../../../src/source.ts";

  import { Icon } from "../../Primitive";
  import { Remote } from "../../Component";

  import File from "./File.svelte";

  export let prefix = ""; // start sidebar tree from repository root
  export let name = null;
  export let projectId = null;

  export let expanded = false;
  export let toplevel = false;

  const toggleFolder = () => {
    expanded = !expanded;
  };

  const onFileClick = filePath => {
    updateParams({
      path: filePath,
      projectId: projectId,
      revision: $currentRevision,
      type: ObjectType.Blob,
    });
  };

  $: store = tree(projectId, $currentRevision, prefix);
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

<Remote {store} let:data={tree}>
  {#if !toplevel}
    <div class="folder" on:click={toggleFolder}>
      <span class:expanded style="height: 24px">
        <Icon.Chevron dataCy={`expand-${name}`} />
      </span>
      <span class="folder-name">{name}</span>
    </div>
  {/if}

  <div class="container" class:toplevel>
    {#if expanded || toplevel}
      {#each tree.entries as entry}
        {#if entry.info.objectType === ObjectType.Tree}
          <svelte:self
            {projectId}
            name={entry.info.name}
            prefix={`${entry.path}/`} />
        {:else}
          <File
            name={entry.info.name}
            active={entry.path === $currentPath}
            on:click={() => {
              onFileClick(entry.path);
            }} />
        {/if}
      {/each}
    {/if}
  </div>
</Remote>
