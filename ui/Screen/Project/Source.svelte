<script>
  import { getContext } from "svelte";
  import { push, location } from "svelte-spa-router";
  import { format } from "timeago.js";

  import * as path from "../../src/path.ts";
  import { project as projectStore } from "../../src/project.ts";
  import {
    currentPath,
    currentRevision,
    fetchCommits,
    fetchRevisions,
    commits as commitsStore,
    object as objectStore,
    ObjectType,
    readme,
    revisions as revisionsStore,
    updateParams,
  } from "../../src/source.ts";

  import { Code, Flex, Icon, Text, Title } from "../../DesignSystem/Primitive";
  import { Copyable, Placeholder, Remote } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";

  import CloneButton from "./CloneButton.svelte";

  export const params = null;

  const navigateOnReady = (path, store) => {
    const unsubscribe = store.subscribe(state => {
      if (state.status === "SUCCESS") {
        push(path);
      }
    });
    unsubscribe();
  };

  const updateRevision = (projectId, revision) => {
    updateParams({
      path: path.extractProjectSourceObjectPath($location),
      projectId: projectId,
      revision: revision,
      type: path.extractProjectSourceObjectType($location),
    });
  };

  let copyIcon = Icon.Copy;

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 1000);
  };

  const { id, metadata } = getContext("project");

  const getRevision = current => {
    return current !== "" ? current : metadata.defaultBranch;
  };

  $: updateParams({
    path: path.extractProjectSourceObjectPath($location),
    projectId: id,
    revision: getRevision($currentRevision),
    type: path.extractProjectSourceObjectType($location),
  });
  $: fetchCommits({ projectId: id, branch: getRevision($currentRevision) });

  fetchRevisions({ projectId: id });
</script>

<style>
  .header {
    padding: var(--content-padding);
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
    margin-bottom: 4rem;
    padding: 0 var(--content-padding);
  }

  .column-left {
    display: flex;
    flex-direction: column;
    width: 18rem;
    padding-right: 0.75rem;
  }

  .column-right {
    display: flex;
    flex-direction: column;
    padding-left: 0.75rem;
    min-width: var(--content-min-width);
    width: 100%;
  }

  .commit-header {
    height: 2.5rem;
    margin-bottom: 1rem;
  }

  .source-tree {
    overflow-x: auto;
  }

  .revision-selector-wrapper {
    margin: var(--content-padding) 0;
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
    height: 2.5rem;
    margin: 1.5rem 0;
    display: flex;
    justify-content: space-evenly;
  }
  .repo-stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem 1rem;
    margin-right: 1rem;
  }
  .stat {
    font-family: var(--typeface-mono-bold);
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    border-radius: 0.75rem;
  }
</style>

<Remote store={projectStore} let:data={project}>
  <div class="header">
    <Title variant="big">{project.metadata.name}</Title>
    <div class="project-id">
      <Code>
        <Copyable {afterCopy}>
          {project.shareableEntityIdentifier}
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
      <!-- Revision selector -->
      <Remote store={revisionsStore} let:data={revisions}>
        <div class="revision-selector-wrapper">
          <RevisionSelector
            style="height: 100%;"
            currentRevision={getRevision($currentRevision)}
            {revisions}
            on:select={event => updateRevision(project.id, event.detail)} />
        </div>
      </Remote>

      <!-- Tree -->
      <div class="source-tree" data-cy="source-tree">
        <Folder projectId={project.id} toplevel name={project.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      <div class="repo-header">
        <div class="repo-stats">
          <div class="repo-stat-item">
            <Icon.Commit />
            <Text style="margin: 0 8px;">
              <!-- svelte-ignore a11y-missing-attribute -->
              <a
                data-cy="commits-button"
                on:click={navigateOnReady(path.projectCommits(id, $currentRevision), commitsStore)}>
                Commits
              </a>
            </Text>
            <span class="stat">{project.stats.commits}</span>
          </div>
          <div class="repo-stat-item">
            <Icon.Branch />
            <Text style="margin: 0 8px;">Branches</Text>
            <span class="stat">{project.stats.branches}</span>
          </div>
          <div class="repo-stat-item">
            <Icon.Member />
            <Text style="margin: 0 8px;">Contributors</Text>
            <span class="stat">{project.stats.contributors}</span>
          </div>
        </div>
        <CloneButton projectId={project.id} />
      </div>

      <!-- Object -->
      <Remote store={objectStore} let:data={object}>
        {#if object.info.objectType === ObjectType.Blob}
          <FileSource
            blob={object}
            path={$currentPath}
            rootPath={path.projectSource(project.id)}
            projectName={project.metadata.name}
            projectId={project.id} />
        {:else if object.path === ''}
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

          <!-- Readme -->
          <Remote
            store={readme(id, getRevision($currentRevision))}
            let:data={readme}>
            {#if readme}
              <Readme content={readme.content} path={readme.path} />
            {:else}
              <Flex align="center">
                <Placeholder style="width: 300px; height: 100px" />
                <Text>No readme found placeholder.</Text>
              </Flex>
            {/if}
          </Remote>
        {/if}
      </Remote>
    </div>
  </div>
</Remote>
