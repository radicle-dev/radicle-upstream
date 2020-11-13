<script lang="typescript">
  import type { Readable } from "svelte/store";
  import { push } from "svelte-spa-router";
  import { format } from "timeago.js";

  import * as path from "../../../src/path";
  import type { Project } from "../../../src/project";
  import { selectPath, ViewKind } from "../../../src/screen/project/source";
  import type { Code } from "../../../src/screen/project/source";

  import EmptyState from "../../../DesignSystem/Component/EmptyState.svelte";

  import CommitTeaser from "../../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import FileSource from "../../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Readme from "../../../DesignSystem/Component/SourceBrowser/Readme.svelte";

  export let code: Readable<Code>;
  export let filePath: Readable<string>;
  export let project: Project;

  const onSelectCommit = ({ detail: sha }: { detail: string }) => {
    push(path.projectSourceCommit(project.urn, sha));
  };
  const onRoot = () => selectPath("");
</script>

<style>
  .commit-header {
    height: 2.5rem;
    margin-bottom: 1rem;
  }
</style>

<div data-cy="file-view">
  <div class="commit-header">
    <CommitTeaser
      message={$code.lastCommit.summary}
      on:select={onSelectCommit}
      sha={$code.lastCommit.sha1}
      style="height: 100%"
      timestamp={format($code.lastCommit.committerTime * 1000)}
      user={$code.lastCommit.author} />
  </div>

  {#if $code.view.kind === ViewKind.Blob}
    <FileSource
      blob={$code.view.blob}
      path={$filePath}
      projectName={project.metadata.name}
      on:root={onRoot} />
  {:else if $code.view.kind === ViewKind.Root}
    {#if $code.view.readme}
      <Readme
        content={$code.view.readme.content}
        path={$code.view.readme.path} />
    {:else}
      <EmptyState
        text="This project doesn't have a README yet."
        emoji="👀"
        style="height: 320px;" />
    {/if}
  {:else if $code.view.kind === ViewKind.Error}
    <EmptyState
      emoji="👀"
      headerText={$code.view.error}
      on:primaryAction={onRoot}
      primaryActionText="Back to source"
      style="height: 320px;"
      text="This file doesn't exist on this branch." />
  {/if}
</div>
