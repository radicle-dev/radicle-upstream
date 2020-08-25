<script>
  import { getContext } from "svelte";
  import { querystring, push } from "svelte-spa-router";
  import { format } from "timeago.js";

  import { openPath } from "../../../native/ipc.js";

  import { checkout } from "../../src/project.ts";
  import * as notification from "../../src/notification.ts";
  import * as path from "../../src/path.ts";
  import * as screen from "../../src/screen.ts";
  import { project as projectStore } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import { Variant as IllustrationVariant } from "../../src/illustration.ts";
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

  let scrollY = 0;
  let headerHeight;
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
      screen.lock();
      const path = await checkout(
        id,
        event.detail.checkoutDirectoryPath,
        "PEER_ID_GOES_HERE",
        "BRANCH_TO_CHECK_OUT_GOES_HERE"
      );

      notification.info(
        `${metadata.name} checked out to ${path}`,
        true,
        "Open folder",
        () => {
          openPath(path);
        }
      );
    } catch (error) {
      notification.error(`Checkout failed: ${error.message}`, true);
    } finally {
      screen.unlock();
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
  .header-wrapper {
    background-color: var(--color-foreground-level-1);
  }
  .header {
    padding: var(--content-padding);
  }
  .project-id {
    color: var(--color-foreground-level-5);
    margin-top: 0.5rem;
    display: inline-block;
  }
  .description {
    margin: 0.5rem 0;
  }
  .center-content {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .repo-header-wrapper {
    position: sticky;
    top: var(--topbar-height);
    background-color: var(--color-background);
  }

  .elevation {
    box-shadow: var(--elevation-low);
  }

  .repo-header {
    display: flex;
    align-items: center;
    height: 4rem;
    padding: 0 var(--content-padding);
  }

  .header-right {
    display: flex;
    justify-content: space-between;
    width: 100%;
    align-items: center;
  }

  .revision-selector-wrapper {
    min-width: 18rem;
    margin: var(--content-padding) 0;
    position: relative;
    padding-right: 0.75rem;
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

  .container {
    display: flex;
    width: inherit;
    margin-bottom: 4rem;
    padding: 0 var(--content-padding);
  }

  .column-left {
    display: flex;
    flex-direction: column;
    min-width: 18rem;
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
</style>

<svelte:window bind:scrollY />

<Remote store={projectStore} let:data={project}>
  <div class="header-wrapper">
    <div bind:clientHeight={headerHeight} class="header center-content">
      <h2>{project.metadata.name}</h2>
      <div class="description">
        <p>{project.metadata.description}</p>
      </div>
      <div class="project-id">
        <Urn
          urn={project.shareableEntityIdentifier}
          showOnHover
          notificationText="The project ID was copied to your clipboard" />
      </div>
    </div>
  </div>
  <div class="wrapper">
    <div class="repo-header-wrapper" class:elevation={scrollY > headerHeight}>
      <div class="repo-header center-content">
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
        <div class="header-right">
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
              <Icon.User />
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
      </div>
    </div>

    <div class="container center-content">
      <div class="column-left">

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
                  illustration={IllustrationVariant.Eyes}
                  style="height: 320px;" />
              {/if}
            </Remote>
          {/if}
        </Remote>
      </div>
    </div>
  </div>
</Remote>
