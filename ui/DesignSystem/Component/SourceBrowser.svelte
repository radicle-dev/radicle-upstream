<script>
  import { getContext } from "svelte";

  import { BLOB, TREE } from "../../../native/types.js";
  import {
    currentRevision,
    fetchRevisions,
    object,
    revisions,
    updateRevision
  } from "../../src/source.ts";

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

  $: console.log("currentRevision", $currentRevision);
  $: console.log("object", $object);

  fetchRevisions({ projectId: projectId });
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
    {#if $revisions.status === 'NOT_ASKED'}
      <p>Not asked...</p>
    {:else if $revisions.status === 'LOADING'}
      <p>Loading...</p>
    {:else if $revisions.status === 'SUCCESS'}
      <Input.Dropdown
        dataCy="revision-selector"
        style="margin-bottom: 24px"
        items={$revisions.data.branches}
        on:select={event => updateRevision({
            revision: event.detail,
            projectId: projectId
          })} />
    {:else if $revisions.status === 'ERROR'}
      <p>{`error: ${$object.error.message}`}</p>
    {/if}

    {#await $project then result}
      <div class="source-tree" data-cy="source-tree">
        <Folder name={result.data.project.metadata.name} />
      </div>
    {/await}
  </div>

  <div class="column-right">
    {#if $object.status === 'LOADING'}
      <p>Loading...</p>
    {:else if $object.status === 'SUCCESS'}
      {#if $object.data.info.objectType === BLOB}
        <FileSource blob={$object.data} {projectId} />
      {:else if $object.data.info.objectType === TREE}
        <FileList {projectId} tree={$object.data} revision={$currentRevision} />
      {/if}
    {:else if $object.status === 'ERROR'}
      <p>{`error: ${$object.error.message}`}</p>
    {/if}
  </div>
</div>
