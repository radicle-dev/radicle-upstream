<script>
  import { getContext } from "svelte";

  import { BLOB, TREE } from "../../../native/types.js";
  import { source, updateRevision } from "../../src/sourceBrowser.ts";

  import { Input } from "../Primitive";
  import FileList from "./SourceBrowser/FileList.svelte";
  import FileSource from "./SourceBrowser/FileSource.svelte";
  import Folder from "./SourceBrowser/Folder.svelte";

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

{#if $source.status === 'LOADING'}
  <p>loading.....</p>
{:else if $source.status === 'SUCCESS'}
  <div class="container" {style}>
    <div class="column-left">
      <Input.Dropdown
        dataCy="revision-selector"
        style="margin-bottom: 24px"
        items={$source.data.revisions.branches}
        on:select={revision => updateRevision({ newRevision: revision })} />
      {#await $project then result}
        <div class="source-tree" data-cy="source-tree">
          <Folder name={result.data.project.metadata.name} />
        </div>
      {/await}
    </div>

    <div class="column-right">
      {#if $source.data.sourceObject.type === BLOB}
        <FileSource blob={$source.data.sourceObject} {projectId} />
      {:else if $source.data.sourceObject.type === TREE}
        <FileList
          {projectId}
          tree={$source.data.sourceObject}
          revision={$source.data.currentRevision} />
      {/if}
    </div>
  </div>
{:else if $source.status === 'ERROR'}
  <p>{`error: ${$source.error.message}`}</p>
{/if}
