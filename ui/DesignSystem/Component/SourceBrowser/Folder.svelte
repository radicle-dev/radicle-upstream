<script lang="typescript">
  import type { PeerId } from "../../../src/identity";
  import * as path from "../../../src/path";
  import { selectPath } from "../../../src/screen/project/source";
  import { tree, objectPath, ObjectType } from "../../../src/source";
  import type { Revision } from "../../../src/source";
  import * as urn from "../../../src/urn";

  import { Icon } from "../../Primitive";
  import { Remote } from "../../Component";

  import File from "./File.svelte";

  export let name: string = "";
  export let peerId: PeerId;
  export let projectUrn: urn.Urn;
  export let revision: Revision;

  export let expanded: boolean = false;
  export let toplevel: boolean = false;

  // Starting-point of this recursive component, empty string means that it
  // starts the sidebar tree from the repository root. This prop should not be
  // used from outside of the component.
  export let prefix: string = "";

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
  .container.toplevel {
    display: inline-block;
    padding-left: 0;
  }
</style>

{#if !toplevel}
  <div class="folder" on:click={toggle}>
    <span class:active style="height: 1.5rem">
      <svelte:component
        this={expanded ? Icon.ChevronDown : Icon.ChevronRight}
        dataCy={`expand-${name}`} />
    </span>
    <span class="folder-name">{name}</span>
  </div>
{/if}

<div class="container" class:toplevel>
  {#if expanded || toplevel}
    <Remote {store} let:data={tree}>
      {#each tree.entries as entry}
        {#if entry.info.objectType === ObjectType.Tree}
          <svelte:self
            name={entry.info.name}
            {peerId}
            prefix={`${entry.path}/`}
            {projectUrn}
            {revision} />
        {:else}
          <File
            active={entry.path === $objectPath}
            href={path.projectSourceCode(projectUrn)}
            name={entry.info.name}
            on:click={() => {
              selectPath(entry.path);
            }} />
        {/if}
      {/each}
    </Remote>
  {/if}
</div>
