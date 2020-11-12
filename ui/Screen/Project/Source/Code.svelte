<script lang="typescript">
  import { format } from "timeago.js";

  import {
    code,
    CodeView,
    selectPath,
    store,
  } from "../../../src/screen/project/source";

  import CommitTeaser from "../../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import EmptyState from "../../../DesignSystem/Component/Remote.svelte";
  import FileSource from "../../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Folder from "../../../DesignSystem/Component/SourceBrowser/Folder.svelte";
  import Readme from "../../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Remote from "../../../DesignSystem/Component/Remote.svelte";

  const onRoot = () => selectPath("");
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
    <Remote {store} let:data={{ kind, project, selectedPeer }}>
      <Remote
        store={code}
        let:data={{ file, error, lastCommit, path, readme, selectedRevision }}>
        <div class="column-left">
          <!-- Tree -->
          <div class="source-tree" data-cy="source-tree">
            <Folder
              peerId={selectedPeer.peerId}
              projectUrn={project.urn}
              revision={selectedRevision}
              toplevel />
          </div>
        </div>

        <div class="column-right">
          {#if kind === CodeView.File || kind === CodeView.Root}
            <div class="commit-header">
              <CommitTeaser
                message={lastCommit.summary}
                sha={lastCommit.sha1}
                projectUrn={project.urn}
                style="height: 100%"
                timestamp={format(lastCommit.committerTime * 1000)}
                user={lastCommit.author} />
            </div>
          {/if}

          {#if kind === CodeView.File}
            <FileSource
              blob={file}
              {path}
              projectName={project.metadata.name}
              on:root={onRoot} />
          {:else if kind === CodeView.Root}
            {#if readme}
              <Readme content={readme.content} path={readme.path} />
            {:else}
              <EmptyState
                text="This project doesn't have a README yet."
                emoji="👀"
                style="height: 320px;" />
            {/if}
          {:else if kind === CodeView.Error}
            <EmptyState
              emoji="👀"
              headerText={error}
              on:primaryAction={onRoot}
              primaryActionText="Back to source"
              style="height: 320px;"
              text="This file doesn't exist on this branch." />
          {/if}
        </div>
      </Remote>
    </Remote>
  </div>
</div>
