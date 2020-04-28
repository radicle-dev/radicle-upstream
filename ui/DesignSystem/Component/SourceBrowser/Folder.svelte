<script>
  import { link } from "svelte-spa-router";

  import { TREE } from "../../../../native/types.js";
  import * as path from "../../../lib/path.js";
  import { currentPath, currentRevision, tree } from "../../../src/source.ts";

  import { Icon } from "../../Primitive";
  import { Remote } from "../../Component";

  import File from "./File.svelte";

  export let prefix = ""; // start sidebar tree from repository root
  export let name = null;
  export let projectId = null;

  export let expanded = false;
  export let firstEntry = true;

  const toggle = () => {
    expanded = !expanded;
  };

  $: store = tree(projectId, $currentRevision, prefix);
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

<Remote {store} let:data={tree}>
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
      {#each tree.entries as entry}
        {#if entry.info.objectType === TREE}
          <svelte:self
            {projectId}
            name={entry.info.name}
            prefix={`${entry.path}/`}
            firstEntry={false} />
        {:else}
          <File {projectId} filePath={entry.path} name={entry.info.name} />
        {/if}
      {/each}
    {/if}
  </div>
</Remote>
