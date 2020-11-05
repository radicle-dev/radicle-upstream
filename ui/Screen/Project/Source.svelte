<script lang="typescript">
  import { getContext } from "svelte";
  import { format } from "timeago.js";

  import {
    fetchObject,
    object as store,
    selectedPeer,
    selectedRevision,
  } from "../../src/project";
  import type { Project } from "../../src/project";
  import { ObjectType, objectPath, objectType, readme } from "../../src/source";
  import * as urn from "../../src/urn";

  import { EmptyState, Remote } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";

  export let params: { urn: urn.Urn };
  const project: Project = getContext("project");

  $: if ($selectedPeer && $selectedRevision) {
    fetchObject(
      $objectType,
      project.urn,
      $selectedPeer.peerId,
      $objectPath,
      $selectedRevision
    );
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
      {#if $selectedPeer && $selectedRevision}
        <!-- Tree -->
        <div class="source-tree" data-cy="source-tree">
          <Folder
            peerId={$selectedPeer.peerId}
            projectUrn={project.urn}
            revision={$selectedRevision}
            toplevel />
        </div>
      {/if}
    </div>

    <div class="column-right">
      <!-- Object -->
      <Remote {store} let:data={object}>
        {#if object.info.objectType === ObjectType.Blob}
          <FileSource
            blob={object}
            path={$objectPath}
            projectName={project.metadata.name}
            projectUrn={project.urn} />
        {:else if object.path === ''}
          <!-- Repository root -->
          <div class="commit-header">
            <CommitTeaser
              message={object.info.lastCommit.summary}
              sha={object.info.lastCommit.sha1}
              projectUrn={project.urn}
              style="height: 100%"
              timestamp={format(object.info.lastCommit.committerTime * 1000)}
              user={object.info.lastCommit.author} />
          </div>

          <!-- Readme -->
          <Remote
            store={readme(project.urn, $selectedPeer.peerId, $selectedRevision)}
            let:data={readme}>
            {#if readme}
              <Readme content={readme.content} path={readme.path} />
            {:else}
              <EmptyState
                text="This project doesn't have a README yet."
                emoji="👀"
                style="height: 320px;" />
            {/if}
          </Remote>
        {/if}
        <div slot="error" let:error>
          <EmptyState
            emoji="👀"
            headerText={error.message}
            on:primaryAction={ev => console.log('primary action', ev)}
            primaryActionText="Back to source"
            style="height: 320px;"
            text="This file doesn't exist on this branch." />
        </div>
      </Remote>
    </div>
  </div>
</div>
