<script>
  import { location } from "svelte-spa-router";

  import { BLOB, TREE } from "../../../native/types.js";
  import * as path from "../../lib/path.js";
  import { project } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import {
    currentPath,
    currentRevision,
    fetchRevisions,
    object,
    revisions,
    updateParams
  } from "../../src/source.ts";

  import { Input, Text, Title } from "../../DesignSystem/Primitive";

  import FileList from "../../DesignSystem/Component/SourceBrowser/FileList.svelte";
  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";

  export const params = null;

  const updateRevision = (projectId, revision) => {
    updateParams({
      path: path.extractProjectSourceObjectPath($location),
      projectId: projectId,
      revision: revision,
      type: path.extractProjectSourceObjectType($location)
    });
  };

  $: if ($project.status === remote.Status.Success) {
    const { id, metadata } = $project.data;

    fetchRevisions({ projectId: id });
    updateParams({
      path: path.extractProjectSourceObjectPath($location),
      projectId: id,
      revision:
        $currentRevision !== "" ? $currentRevision : metadata.defaultBranch,
      type: path.extractProjectSourceObjectType($location)
    });
  }
</script>

<style>
  .header {
    margin-bottom: 32px;
  }

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

{#if $project.status === remote.Status.Success}
  <div class="header">
    <Title variant="big">{$project.data.metadata.name}</Title>
    <Text>{$project.data.metadata.description}</Text>
  </div>

  <div class="container">
    <div class="column-left">
      {#if $revisions.status === remote.Status.Success}
        <Input.Dropdown
          dataCy="revision-selector"
          style="margin-bottom: 24px"
          items={[...$revisions.data.tags, ...$revisions.data.branches]}
          value={$project.data.metadata.defaultBranch}
          on:select={event => updateRevision($project.data.id, event.detail)} />
      {/if}

      <div class="source-tree" data-cy="source-tree">
        <Folder
          projectId={$project.data.id}
          name={$project.data.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      {#if $object.status === remote.Status.Success}
        {#if $object.data.info.objectType === BLOB}
          <FileSource
            blob={$object.data}
            path={$currentPath}
            projectId={$project.id} />
        {:else if $object.data.info.objectType === TREE}
          <FileList
            projectId={$project.id}
            tree={$object.data}
            revision={$currentRevision} />
        {/if}
      {/if}
    </div>
  </div>
{/if}
