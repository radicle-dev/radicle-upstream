<script lang="typescript">
  import { push } from "svelte-spa-router";

  import * as path from "../../../src/path";
  import { selectPath, store } from "../../../src/screen/project/source";
  import { fetchTree } from "../../../src/source";
  import type { Sha1 } from "../../../src/source";
  import type { Urn } from "../../../src/urn";

  import Remote from "../../../DesignSystem/Component/Remote.svelte";

  import FileView from "../../../DesignSystem/Component/SourceBrowser/FileView.svelte";
  import Tree from "../../../DesignSystem/Component/SourceBrowser/Tree.svelte";

  const onSelectCommit = (projectUrn: Urn, sha1: Sha1) => {
    push(path.projectSourceCommit(projectUrn, sha1));
  };
  const onSelectPath = ({ detail: path }: { detail: string }) => {
    selectPath(path);
  };
  const onSelectRoot = () => selectPath("");
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
      let:data={{ code, peer, project, selectedPath, selectedRevision, tree }}>
      <div class="column-left">
        <div class="source-tree" data-cy="source-tree">
          <Tree
            fetchTree={path => fetchTree(project.urn, peer.peerId, selectedRevision.selected, path)}
            on:select={onSelectPath}
            {selectedPath}
            {selectedRevision}
            {tree} />
        </div>
      </div>
      <div class="column-right">
        <FileView
          {code}
          on:commit={({ detail: sha1 }) => onSelectCommit(project.urn, sha1)}
          on:root={onSelectRoot} />
      </div>
    </Remote>
  </div>
</div>
