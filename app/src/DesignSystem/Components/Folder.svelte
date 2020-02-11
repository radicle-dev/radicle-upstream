<script>
  import { gql } from "apollo-boost";
  import { getContext } from "svelte";
  import { getClient, query } from "svelte-apollo";
  import { link } from "svelte-spa-router";

  import { TREE } from "../../types.js";
  import { Icon } from "../Primitives";
  import File from "./File.svelte";
  import { revision, objectPath } from "../../stores.js";
  import * as path from "../../path.js";

  export let prefix = ""; // start sidebar tree from repository root
  export let name = null;

  export let expanded = false;
  export let firstEntry = true;

  const projectId = getContext("projectId");

  const QUERY = gql`
    query Query($projectId: ID!, $revision: String!, $prefix: String!) {
      tree(id: $projectId, revision: $revision, prefix: $prefix) {
        entries {
          path
          info {
            objectType
            name
          }
        }
      }
    }
  `;

  $: sourceTree = query(getClient(), {
    query: QUERY,
    variables: { projectId: projectId, revision: $revision, prefix: prefix }
  });

  let toggle = () => {
    expanded = !expanded;
  };

  $: active = prefix === $objectPath;
</script>

<style>
  .folder {
    display: flex;
    cursor: pointer;
    margin: 0 4px 12px 0;
    color: var(--color-darkgray);
    user-select: none;
  }

  .expanded :global(svg) {
    transform: rotate(90deg);
  }

  .folder :global(svg:hover) {
    background-color: #eeeeef;
    border-radius: 2px;
  }

  .container {
    margin: 0 0 0 8px;
  }

  a {
    display: flex;
  }

  .active a {
    color: var(--color-purple);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-purple);
  }
</style>

{#await $sourceTree then result}
  <div class="container">
    {#if firstEntry}
      <div class="folder" class:active>
        <Icon.Folder dataCy={`expand-${name}`} />
        <a
          href={path.projectSource(projectId, $revision, TREE, prefix)}
          use:link>
          {name}
        </a>
      </div>
    {:else}
      <div class="folder" class:expanded class:active>
        <Icon.CarretBig dataCy={`expand-${name}`} on:click={toggle} />
        <a
          href={path.projectSource(projectId, $revision, TREE, prefix)}
          use:link>
          {name}
        </a>
      </div>
    {/if}

    {#if expanded || firstEntry}
      {#each result.data.tree.entries as entry}
        {#if entry.info.objectType === TREE}
          <svelte:self
            prefix={entry.path + '/'}
            name={entry.info.name}
            firstEntry={false} />
        {:else}
          <File name={entry.info.name} filePath={entry.path} />
        {/if}
      {/each}
    {/if}
  </div>
{/await}
