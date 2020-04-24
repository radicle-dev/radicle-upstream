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
    findReadme,
    blob,
    updateParams
  } from "../../src/source.ts";

  import { Code, Icon, Text, Title } from "../../DesignSystem/Primitive";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";
  import Stat from "../../DesignSystem/Component/Stat.svelte";

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

  let readmePath = null;

  $: readme = null;
  $: if ($object.status === remote.Status.Success) {
    if ($object.data.info.objectType === TREE) {
      readmePath = findReadme($object.data);

      if (path) {
        readme = blob($project.data.id, $currentRevision, readmePath);
      }
    }
  }
</script>

<style>
  .header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  .project-id {
    color: var(--color-foreground-level-5);
    margin-top: 0.5rem;
  }
  .description {
    margin-top: 1rem;
  }

  .container {
    display: flex;
    width: inherit;
  }

  .column-left {
    display: flex;
    flex-direction: column;
    width: 286px;
    padding: 0 0.75rem;
  }

  .column-right {
    display: flex;
    flex-direction: column;
    padding-left: 0.75rem;
    width: 960px;
  }

  .source-tree {
    overflow-x: auto;
  }

  .revision-selector-wrapper {
    margin: 0.75rem 0;
    position: relative;
    width: 100%;
  }
  .repo-stats {
    height: 4rem;
    display: flex;
    justify-content: space-evenly;
    padding: 1.25rem 1rem;
  }
  .repo-stats > * {
    flex: 1;
    color: var(--color-foreground-level-6);
  }
</style>

{#if $project.status === remote.Status.Success}
  <div class="header">
    <Title variant="big">{$project.data.metadata.name}</Title>
    <div class="project-id">
      <Code>%{$project.data.id}</Code>
    </div>
    <div class="description">
      <Text>{$project.data.metadata.description}</Text>
    </div>
  </div>

  <div class="container">
    <div class="column-left">
      {#if $revisions.status === remote.Status.Success}
        <div class="revision-selector-wrapper">
          <RevisionSelector
            style="height: 100%;"
            currentRevision={$currentRevision}
            revisions={$revisions.data}
            on:select={event => updateRevision($project.data.id, event.detail)} />
        </div>
      {/if}

      <div class="source-tree" data-cy="source-tree">
        <Folder
          projectId={$project.data.id}
          toplevel
          name={$project.data.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      <div class="repo-stats">
        <div>
          <Stat icon={Icon.Commit} count={$project.data.stats.commits}>
            &nbsp;Commits
          </Stat>
        </div>
        <div>
          <Stat icon={Icon.Branch} count={$project.data.stats.branches}>
            &nbsp;Branches
          </Stat>
        </div>
        <div>
          <Stat icon={Icon.Member} count={$project.data.stats.contributors}>
            &nbsp;Contributors
          </Stat>
        </div>
      </div>
      {#if $object.status === remote.Status.Success}
        {#if $object.data.info.objectType === BLOB}
          <FileSource
            blob={$object.data}
            path={$currentPath}
            projectId={$project.data.id} />
        {:else if $object.data.info.objectType === TREE && $readme.status === remote.Status.Success}
          <Readme
            path={readmePath}
            commit={$object.data.info.lastCommit}
            blob={$readme.data}
            projectId={$project.data.id} />
        {/if}
      {/if}
    </div>
  </div>
{/if}
