<script>
  import { getContext } from "svelte";

  import {
    objectPathStore,
    objectTypeStore,
    revisionStore
  } from "../../stores/sourceBrowsing.js";
  import { BLOB } from "../../types.js";
  import {
    FileList,
    FileSource,
    Folder,
    RevisionSelector
  } from "../../DesignSystem/Components";

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

<div class="container">
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
