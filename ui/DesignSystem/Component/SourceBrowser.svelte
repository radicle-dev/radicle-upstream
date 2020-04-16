<script>
  import { getContext } from "svelte";

  import { sourceBrowser, updateRevision } from "../../src/sourceBrowser.ts";

  import { BLOB, TREE } from "../../../native/types.js";
  import FileList from "./SourceBrowser/FileList.svelte";
  import FileSource from "./SourceBrowser/FileSource.svelte";
  import Folder from "./SourceBrowser/Folder.svelte";
  import RevisionSelector from "./SourceBrowser/RevisionSelector.svelte";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const projectId = getContext("projectId");

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

{#if $sourceBrowser.status === 'LOADING'}
  <p>loading.....</p>
{:else if $sourceBrowser.status === 'SUCCESS'}
  <div class="container" {style}>
    <div class="column-left">
      <RevisionSelector
        revisions={$sourceBrowser.data.revisions.branches}
        onSelect={revision => updateRevision({ newRevision: revision })} />
      {#await $project then result}
        <div class="source-tree" data-cy="source-tree">
          <Folder name={result.data.project.metadata.name} />
        </div>
      {/await}
    </div>

    <div class="column-right">
      {#if $sourceBrowser.data.sourceObject.type === BLOB}
        <FileSource blob={$sourceBrowser.data.sourceObject} {projectId} />
      {:else if $sourceBrowser.data.sourceObject.type === TREE}
        <FileList
          {projectId}
          tree={$sourceBrowser.data.sourceObject}
          revision={$sourceBrowser.data.currentRevision} />
      {/if}
    </div>
  </div>
{:else if $sourceBrowser.status === 'ERROR'}
  <p>{`error: ${$sourceBrowser.error.message}`}</p>
{/if}
