<script lang="typescript">
  import { selectPath, store } from "../../../src/screen/project/source";

  import Remote from "../../../DesignSystem/Component/Remote.svelte";

  import TreeRoot from "../../../DesignSystem/Component/SourceBrowser/TreeRoot.svelte";

  import FileView from "./FileView.svelte";

  const onSelectPath = ({ detail: path }: { detail: string }) => {
    selectPath(path);
  };
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

  .source-tree {
    overflow-x: auto;
    width: 18rem;
  }
</style>

<div class="wrapper">
  <div class="container center-content">
    <Remote
      {store}
      let:data={{ code, path, peer, project, selectedRevision, tree }}>
      <div class="column-left">
        <div class="source-tree" data-cy="source-tree">
          <TreeRoot
            currentPath={path}
            on:select={onSelectPath}
            peerId={peer.peerId}
            projectUrn={project.urn}
            revision={selectedRevision}
            {tree} />
        </div>
      </div>

      <div class="column-right">
        <FileView {code} filePath={path} {project} />
      </div>
    </Remote>
  </div>
</div>
