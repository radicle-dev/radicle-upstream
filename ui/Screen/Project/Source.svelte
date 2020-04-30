<script>
  import { location } from "svelte-spa-router";
  import { format } from "timeago.js";

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
  import { Remote, Copyable } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";
  import Stat from "../../DesignSystem/Component/Stat.svelte";

  import CloneButton from "./CloneButton.svelte";

  export const params = null;

  const updateRevision = (projectId, revision) => {
    updateParams({
      path: path.extractProjectSourceObjectPath($location),
      projectId: projectId,
      revision: revision,
      type: path.extractProjectSourceObjectType($location)
    });
  };

  let copyIcon = Icon.Copy;

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 1000);
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

      if (readmePath) {
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
    display: inline-block;
  }
  .description {
    margin-top: 1rem;
  }

  .container {
    display: flex;
    width: inherit;
    margin-bottom: 64px;
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

  .commit-header {
    height: 3rem;
    background-color: var(--color-secondary-level-1);
    margin-bottom: 1rem;
    border-radius: 3px;
  }

  .source-tree {
    overflow-x: auto;
  }

  .revision-selector-wrapper {
    margin: 0.75rem 0;
    position: relative;
    width: 100%;
  }
  .repo-header {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .repo-stats {
    height: 4rem;
    display: flex;
    justify-content: space-evenly;
    padding: 1.25rem 1rem;
    flex: 0.8;
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
      <Code>
        <Copyable {afterCopy}>
          %{project.id}
          <svelte:component this={copyIcon} style="vertical-align: bottom" />
        </Copyable>
      </Code>
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
      <div class="repo-header">
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
        <CloneButton projectId={project.id} />
      </div>

      <Remote store={objectStore} let:data={object}>
        {#if object.info.objectType === BLOB}
          <FileSource
            blob={object}
            path={$currentPath}
            rootPath={path.projectSource(project.id)}
            projectName={project.metadata.name}
            projectId={project.id} />
        {:else if object.info.objectType === TREE && !object.path}
          <!-- Repository root -->
          <div class="commit-header">
            <CommitTeaser
              projectId={project.id}
              user={{ username: object.info.lastCommit.author.name, avatar: object.info.lastCommit.author.avatar }}
              commitMessage={object.info.lastCommit.summary}
              commitSha={object.info.lastCommit.sha1}
              timestamp={format(object.info.lastCommit.committerTime * 1000)}
              style="height: 100%" />
          </div>

          {#if readmePath}
            <Remote store={readme} let:data={blob}>
              {#if blob.binary}
                <!-- TODO: Placeholder for when README is binary -->
              {:else}
                <Readme
                  {blob}
                  path={readmePath}
                  commit={object.info.lastCommit}
                  projectId={project.id} />
              {/if}
            </Remote>
          {:else}
            <!-- TODO: Placeholder for when projects don't have a README -->
          {/if}
        {/if}
      </Remote>
    </div>
  </div>
</Remote>
