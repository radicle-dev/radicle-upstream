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
      </div>
      <div class="column-right">
        <FileView
          {code}
          {tree}
          on:commit={({ detail: sha1 }) => onSelectCommit(project.urn, sha1)}
          on:root={onSelectRoot} />
      </div>
    </Remote>
  </div>
</div>
