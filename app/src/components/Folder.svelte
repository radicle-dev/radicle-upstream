<script>
  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { query } from "svelte-apollo";

  import { getContext } from "svelte";
  import { link } from "svelte-spa-router";
  import { Icon } from "../DesignSystem";
  import File from "./File.svelte";
  import { revision, objectPath } from "../stores.js";
  import * as path from "../path.js";

  const projectId = getContext("projectId");
  export let prefix = "/";
  export let name = null;

  export let expanded = false;
  export let firstEntry = true;

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  const TREE = gql`
    query Query($projectId: String!, $revision: String!, $prefix: String!) {
      tree(projectId: $projectId, revision: $revision, prefix: $prefix) {
        leaves {
          path
          info {
            isDirectory
            name
          }
        }
      }
    }
  `;

  $: sourceTree = query(client, {
    query: TREE,
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
    font-family: "GT America Medium";
  }

  .active :global(svg) {
    fill: var(--color-purple);
  }
</style>

{#await $sourceTree then result}
  <div class="container">
    {#if !firstEntry}
      <div class="folder" class:expanded class:active>
        <Icon.CarretBig on:click={toggle} />
        <a
          href={path.projectSource({
            id: projectId,
            revision: $revision,
            objectType: 'tree',
            path: prefix
          })}
          use:link>
          {name}
        </a>
      </div>
    {/if}

    {#if expanded || firstEntry}
      {#each result.data.tree.leaves as entry}
        {#if entry.info.isDirectory}
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
