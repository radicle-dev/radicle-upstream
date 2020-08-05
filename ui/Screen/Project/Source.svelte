<script>
  import { getContext } from "svelte";
  import { querystring, push } from "svelte-spa-router";
  import { format } from "timeago.js";

  import { checkout } from "../../src/project.ts";
  import * as notification from "../../src/notification.ts";
  import * as path from "../../src/path.ts";
  import { project as projectStore } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import {
    fetchCommits,
    fetchRevisions,
    commits as commitsStore,
    object as objectStore,
    ObjectType,
    readme,
    revisions as revisionsStore,
    fetchObject,
    revisionQueryEq,
    RevisionType,
  } from "../../src/source.ts";

  import { Icon } from "../../DesignSystem/Primitive";
  import { EmptyState, Remote, Urn } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";

  import CheckoutButton from "./CheckoutButton.svelte";

  const { id, metadata } = getContext("project");

  let currentPeerId;
  let currentRevision;
  let currentObjectType;
  let currentObjectPath;

  $: {
    const parsed = path.parseQueryString($querystring);

    currentPeerId = parsed.peerId || null;
    currentObjectType = parsed.objectType || ObjectType.Tree;
    currentObjectPath = parsed.objectPath || null;

    if (currentRevision && parsed.revision) {
      // Only perform assignment if there is a change to the revision.
      // Otherwise an assignment triggers this reacitve statement to update
      // the source browser even if there are no changes.
      if (!revisionQueryEq(currentRevision, parsed.revision)) {
        currentRevision = parsed.revision;
      }
    } else {
      currentRevision = {
        type: RevisionType.Branch,
        name: metadata.defaultBranch,
        peerId: "",
      };
    }
  }

  const updateRevision = (projectId, revision, peerId) => {
    push(
      path.projectSource(
        projectId,
        peerId,
        revision,
        currentObjectType,
        currentObjectPath
      )
    );
  };

  const handleCheckout = async event => {
    try {
      await checkout(
        id,
        event.detail.checkoutDirectoryPath,
        "PEER_ID_GOES_HERE",
        "BRANCH_TO_CHECK_OUT_GOES_HERE"
      );

      notification.info(
        `${metadata.name} checked out to ${event.detail.checkoutDirectoryPath}`
      );
    } catch (error) {
      notification.error(`Checkout failed: ${error.message}`);
    }
  };

  // TODO(rudolfs): this functionality should be part of navigation/routing.
  let unsubscribe;
  const navigateOnReady = (path, store) => {
    if (unsubscribe) {
      unsubscribe();
    }

    fetchCommits({ projectId: id, revision: currentRevision });

    unsubscribe = store.subscribe(state => {
      if (state.status === remote.Status.Success) {
        push(path);
        unsubscribe();
      }
    });
  };

  $: fetchObject({
    path: currentObjectPath,
    peerId: currentPeerId,
    projectId: id,
    revision: currentRevision,
    type: currentObjectType,
  });

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
    cursor: pointer;
  }
  .repo-stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem 1rem;
    margin-right: 1rem;
  }
  .stat {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    border-radius: 0.75rem;
  }
</style>

<Remote store={projectStore} let:data={project}>
  <div class="header">
    <h2>{project.metadata.name}</h2>
    <div class="project-id">
      <Urn urn={project.shareableEntityIdentifier} showOnHover />
    </div>
    <div class="description">
      <p>{project.metadata.description}</p>
    </div>
  </div>

  <div class="container">
    <div class="column-left">
      <!-- Revision selector -->
      <Remote store={revisionsStore} let:data={revisions}>
        <div class="revision-selector-wrapper">
          <RevisionSelector
            {currentPeerId}
            {currentRevision}
            {revisions}
            on:select={event => {
              updateRevision(project.id, event.detail.revision, event.detail.peerId);
            }} />
        </div>
      </Remote>

      <!-- Tree -->
      <div class="source-tree" data-cy="source-tree">
        <Folder
          {currentRevision}
          {currentObjectPath}
          {currentPeerId}
          projectId={project.id}
          toplevel
          name={project.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      <div class="repo-header">
        <div class="repo-stats" data-cy="repo-stats">
          <div class="repo-stat-item">
            <Icon.Commit />
            <p style="margin: 0 8px;">
              <!-- svelte-ignore a11y-missing-attribute -->
              <a
                data-cy="commits-button"
                on:click={navigateOnReady(path.projectCommits(project.id, currentRevision), commitsStore)}>
                Commits
              </a>
            </p>
            <span class="stat typo-mono-bold">{project.stats.commits}</span>
          </div>
          <div class="repo-stat-item">
            <Icon.Branch />
            <p style="margin: 0 8px;">Branches</p>
            <span class="stat typo-mono-bold">{project.stats.branches}</span>
          </div>
          <div class="repo-stat-item">
            <Icon.Member />
            <p style="margin: 0 8px;">Contributors</p>
            <span class="stat typo-mono-bold">
              {project.stats.contributors}
            </span>
          </div>
        </div>
        <CheckoutButton
          on:checkout={handleCheckout}
          projectName={project.metadata.name} />
      </div>

      <!-- Object -->
      <Remote store={objectStore} let:data={object}>
        {#if object.info.objectType === ObjectType.Blob}
          <FileSource
            blob={object}
            path={currentObjectPath}
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
            store={readme(id, currentPeerId, currentRevision)}
            let:data={readme}>
            {#if readme}
              <Readme content={readme.content} path={readme.path} />
            {:else}
              <EmptyState
                text="This project doesn't have a README yet."
                icon="eyes"
                style="height: 320px;" />
            {/if}
          </Remote>
        {/if}
      </Remote>
    </div>
  </div>
</Remote>
