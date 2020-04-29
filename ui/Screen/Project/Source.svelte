<script>
  import { location } from "svelte-spa-router";

  import { BLOB, TREE } from "../../../native/types.js";
  import * as path from "../../lib/path.js";
  import { project as projectStore } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import {
    currentPath,
    currentRevision,
    fetchRevisions,
    object as objectStore,
    revisions as revisionsStore,
    findReadme,
    blob,
    updateParams
  } from "../../src/source.ts";

  import { Code, Icon, Text, Title } from "../../DesignSystem/Primitive";
  import { Remote } from "../../DesignSystem/Component";

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

  $: if ($projectStore.status === remote.Status.Success) {
    const { id, metadata } = $projectStore.data;

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
  $: if ($objectStore.status === remote.Status.Success) {
    if ($objectStore.data.info.objectType === TREE) {
      readmePath = findReadme($objectStore.data);

      if (path) {
        if ($projectStore.status === remote.Status.Success) {
          readme = blob($projectStore.data.id, $currentRevision, readmePath);
        }
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

<Remote store={projectStore} let:data={project}>
  <div class="header">
    <Title variant="big">{project.metadata.name}</Title>
    <div class="project-id">
      <Code>%{project.id}</Code>
    </div>
    <div class="description">
      <Text>{project.metadata.description}</Text>
    </div>
  </div>

  <div class="container">
    <div class="column-left">
      <Remote store={revisionsStore} let:data={revisions}>
        <div class="revision-selector-wrapper">
          <RevisionSelector
            style="height: 100%;"
            currentRevision={$currentRevision}
            {revisions}
            on:select={event => updateRevision(project.id, event.detail)} />
        </div>
      </Remote>

      <div class="source-tree" data-cy="source-tree">
        <Folder projectId={project.id} toplevel name={project.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      <div class="repo-stats">
        <div>
          <Stat icon={Icon.Commit} count={project.stats.commits}>
            &nbsp;Commits
          </Stat>
        </div>
        <div>
          <Stat icon={Icon.Branch} count={project.stats.branches}>
            &nbsp;Branches
          </Stat>
        </div>
        <div>
          <Stat icon={Icon.Member} count={project.stats.contributors}>
            &nbsp;Contributors
          </Stat>
        </div>
      </div>
      <Remote store={objectStore} let:data={object}>
        {#if object.info.objectType === BLOB}
          <FileSource
            blob={object}
            path={$currentPath}
            projectName={project.metadata.name}
            projectId={project.id} />
        {:else if object.info.objectType === TREE && $readme.status === remote.Status.Success}
          <Readme
            path={readmePath}
            commit={object.info.lastCommit}
            blob={readme.data}
            projectId={project.id} />
        {/if}
      </Remote>
    </div>
  </div>
</Remote>
