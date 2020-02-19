<script>
  import { getContext } from "svelte";

  import {
    objectPathStore,
    objectTypeStore,
    revisionStore
  } from "../../stores/sourceBrowser.js";

  import { BLOB } from "../../lib/types.js";
  import FileList from "./SourceBrowser/FileList.svelte";
  import FileSource from "./SourceBrowser/FileSource.svelte";
  import Folder from "./SourceBrowser/Folder.svelte";
  import RevisionSelector from "./SourceBrowser/RevisionSelector.svelte";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  let projectId = getContext("projectId");

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $projectId) {
        metadata {
          name
        }
      }
    }
  `;

  export let style = null;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: getContext("projectId") }
  });
</script>

<style>
  .container {
    display: flex;
    width: inherit;
  }
  .column-left {
    display: flex;
    flex-direction: column;
    width: 196px;
  }

  .column-right {
    display: flex;
    flex-direction: column;
    width: 960px;
    padding-left: 24px;
  }

  .source-tree {
    overflow-x: scroll;
  }
</style>

<div class="container" {style}>
  <div class="column-left">
    <RevisionSelector />
    {#await $project then result}
      <div class="source-tree" data-cy="source-tree">
        <Folder name={result.data.project.metadata.name} />
      </div>
    {/await}
  </div>

  <div class="column-right">
    {#if $objectTypeStore === BLOB}
      <FileSource
        {projectId}
        path={$objectPathStore}
        revision={$revisionStore} />
    {:else}
      <FileList
        {projectId}
        prefix={$objectPathStore}
        revision={$revisionStore} />
    {/if}
  </div>
</div>
