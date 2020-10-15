<script>
  import { getContext } from "svelte";
  import { format } from "timeago.js";

  import { Variant as IllustrationVariant } from "../../src/illustration.ts";
  import {
    currentPeerId,
    currentRevision,
    object as objectStore,
    ObjectType,
    objectPath,
    objectType,
    readme,
    resetObjectPath,
    resetObjectType,
    fetchObject,
  } from "../../src/source.ts";

  import { EmptyState, Remote } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";

  const project = getContext("project");

  const reset = () => {
    resetObjectPath();
    resetObjectType();
  };

  // Reset some stores on first load
  reset();

  $: if ($currentPeerId) {
    fetchObject({
      path: $objectPath,
      peerId: $currentPeerId,
      projectId: project.id,
      revision: $currentRevision,
      type: $objectType,
    });
  }
</script>

<style>
  .center-content {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
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
    width: 18rem;
  }
</style>

<div class="wrapper">
  <div class="container center-content">
    <div class="column-left">
      {#if $currentPeerId}
        <!-- Tree -->
        <div class="source-tree" data-cy="source-tree">
          <Folder
            currentRevision={$currentRevision}
            currentPeerId={$currentPeerId}
            projectId={project.id}
            toplevel
            name={project.metadata.name} />
        </div>
      {/if}
    </div>

    <div class="column-right">
      <!-- Object -->
      <Remote store={objectStore} let:data={object}>
        {#if object.info.objectType === ObjectType.Blob}
          <FileSource
            blob={object}
            path={$objectPath}
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
            store={readme(project.id, $currentPeerId, $currentRevision)}
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
        <div slot="error" let:error>
          <EmptyState
            headerText={error.message}
            text="This file doesn't exist on this branch."
            primaryActionText="Back to source"
            illustration={IllustrationVariant.Eyes}
            on:primaryAction={reset}
            style="height: 320px;" />
        </div>
      </Remote>
    </div>
  </div>
</div>
