<script>
  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { query } from "svelte-apollo";
  import { link, location } from "svelte-spa-router";
  import { Icon } from "../DesignSystem";
  import File from "./File.svelte";
  import { head } from "../stores.js";

  export let projectId = null;
  export let prefix = "/";
  export let name = null;

  export let expanded = false;
  export let firstEntry = true;

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  const LS = gql`
    query Query($projectId: String!, $head: String!, $prefix: String!) {
      ls(projectId: $projectId, head: $head, prefix: $prefix) {
        path
        info {
          isDirectory
          name
        }
      }
    }
  `;

  $: sourceTree = query(client, {
    query: LS,
    variables: { projectId: projectId, head: $head, prefix: prefix }
  });

  let toggle = () => {
    expanded = !expanded;
  };
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
    background-color: var(--color-lightgray-tint-10);
  }

  .container {
    margin: 0 0 0 8px;
  }

  a {
    display: flex;
  }
</style>

{#await $sourceTree then result}
  <div class="container">
    {#if !firstEntry}
      <div class="folder" class:expanded>
        <Icon.CarretBig on:click={toggle} />
        <a href={$location + prefix} use:link>{name}</a>
      </div>
    {/if}

    {#if expanded || firstEntry}
      {#each result.data.ls as entry}
        {#if entry.info.isDirectory}
          <svelte:self
            {projectId}
            prefix={entry.path + '/'}
            name={entry.info.name}
            firstEntry={false} />
        {:else}
          <File name={entry.info.name} path={entry.path} />
        {/if}
      {/each}
    {/if}
  </div>
{/await}
