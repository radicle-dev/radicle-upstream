<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as router from "ui/src/router";
  import * as proxy from "ui/src/proxy";
  import { selectPath, store } from "ui/src/screen/project/source";

  import FileView from "./SourceBrowser/FileView.svelte";
  import Remote from "ui/App/SharedComponents/Remote.svelte";
  import Tree from "./SourceBrowser/Tree.svelte";

  const onSelectCommit = (projectUrn: string, sha1: string) => {
    router.push({
      type: "project",
      params: {
        urn: projectUrn,
        activeView: { type: "commit", commitHash: sha1 },
      },
    });
  };

  const onSelectPath = ({ detail: path }: { detail: string }) => {
    selectPath(path);
  };
  const onSelectRoot = () => selectPath("");
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);

    display: flex;
    padding: 0 var(--content-padding) 4rem;
    gap: 1.5rem;
  }

  .source-tree {
    overflow-x: auto;
    width: 18rem;
    flex-shrink: 0;
  }

  .file-content {
    flex-grow: 1;
  }
</style>

<div class="container">
  <Remote
    {store}
    let:data={{ code, peer, project, selectedPath, selectedRevision, tree }}>
    <div class="source-tree" data-cy="source-tree">
      <Tree
        fetchTree={path =>
          proxy.client.source.treeGet({
            projectUrn: project.urn,
            peerId: peer.peerId,
            revision: selectedRevision.selected,
            prefix: path,
          })}
        on:select={onSelectPath}
        {selectedPath}
        {selectedRevision}
        {tree} />
    </div>
    <div class="file-content">
      <FileView
        {code}
        {tree}
        on:commit={({ detail: sha1 }) => onSelectCommit(project.urn, sha1)}
        on:root={onSelectRoot} />
    </div>
  </Remote>
</div>
