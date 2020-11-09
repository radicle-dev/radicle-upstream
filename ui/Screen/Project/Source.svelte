<script lang="typescript">
  import { format } from "timeago.js";

  import {
    code as store,
    CodeView,
    selectPath,
  } from "../../src/screen/project";
  import type { Blob } from "../../src/source";

  import { EmptyState, Remote } from "../../DesignSystem/Component";

  import FileSource from "../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import Readme from "../../DesignSystem/Component/SourceBrowser/Readme.svelte";
  import Folder from "../../DesignSystem/Component/SourceBrowser/Folder.svelte";

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
    <Remote {store} let:data={view}>
      <div class="column-left">
        <!-- Tree -->
        <div class="source-tree" data-cy="source-tree">
          <Folder
            peerId={view.peer.peerId}
            projectUrn={view.project.urn}
            revision={view.revision}
            toplevel />
        </div>
      </div>

      <div class="column-right">
        <div class="commit-header">
          <CommitTeaser
            message={view.lastCommit.summary}
            sha={view.lastCommit.sha1}
            projectUrn={view.project.urn}
            style="height: 100%"
            timestamp={format(view.lastCommit.committerTime * 1000)}
            user={view.lastCommit.author} />
        </div>

        {#if view.kind === CodeView.File}
          <FileSource
            blob={view.file}
            path={view.path}
            projectName={view.project.metadata.name}
            on:root={onRoot} />
        {:else if view.kind === CodeView.Root}
          {#if view.readme}
            <Readme content={view.readme.content} path={view.readme.path} />
          {:else}
            <EmptyState
              text="This project doesn't have a README yet."
              emoji="👀"
              style="height: 320px;" />
          {/if}
        {:else if view.kind === CodeView.Error}
          <EmptyState
            emoji="👀"
            headerText={view.file.message}
            on:primaryAction={ev => console.log('primary action', ev)}
            primaryActionText="Back to source"
            style="height: 320px;"
            text="This file doesn't exist on this branch." />
        {/if}
      </div>
    </Remote>
  </div>
</div>
