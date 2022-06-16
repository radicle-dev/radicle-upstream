<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { ObjectType } from "ui/src/source";
  import type { SelectedPath, SelectedRevision, Tree } from "ui/src/source";
  import * as Proxy from "ui/src/proxy";

  import ChevronDownIcon from "design-system/icons/ChevronDown.svelte";
  import ChevronRightIcon from "design-system/icons/ChevronRight.svelte";

  import File from "./File.svelte";

  export let name: string;
  export let path: string;
  export let projectUrn: string;
  export let peerId: string;
  export let selectedRevision: SelectedRevision;
  export let selectedPath: SelectedPath;
  export let selectPath: (path: string) => void;

  let expanded = false;

  const toggle = (): void => {
    expanded = !expanded;
  };

  let current: Promise<Tree>;

  $: {
    const prefix = `${path}/`;
    if (!selectedRevision.request) {
      current = Proxy.client.source.treeGet({
        projectUrn,
        peerId,
        revision: selectedRevision.selected,
        prefix,
      });
    }
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
      this={expanded ? ChevronDownIcon : ChevronRightIcon}
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
            name={entry.info.name}
            path={entry.path}
            {projectUrn}
            {peerId}
            {selectedRevision}
            {selectedPath}
            {selectPath} />
        {:else}
          <File
            active={entry.path === selectedPath.selected}
            dataCy={`file-${entry.path}`}
            loading={entry.path === selectedPath.selected &&
              selectedPath.request !== null}
            name={entry.info.name}
            on:click={() => selectPath(entry.path)} />
        {/if}
      {/each}
    {/if}
  {/await}
</div>
