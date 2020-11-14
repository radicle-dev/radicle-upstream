<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import type { Project } from "../../../src/project";
  import { ViewKind } from "../../../src/screen/project/source";
  import type { Code } from "../../../src/screen/project/source";

  import EmptyState from "../../../DesignSystem/Component/EmptyState.svelte";

  import CommitTeaser from "../../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";
  import FileSource from "../../../DesignSystem/Component/SourceBrowser/FileSource.svelte";
  import Readme from "../../../DesignSystem/Component/SourceBrowser/Readme.svelte";

  export let code: Readable<Code>;
  export let project: Project;

  const dispatch = createEventDispatcher();
  const onSelectCommit = ({ detail: sha1 }: { detail: string }) =>
    dispatch("commit", sha1);
  const onSelectRoot = () => dispatch("root");
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
      commit={$code.lastCommit}
      on:select={onSelectCommit}
      style="height: 100%" />
  </div>

  {#if $code.view.kind === ViewKind.Blob}
    <FileSource
      blob={$code.view.blob}
      path={$code.path}
      projectName={project.metadata.name}
      on:root={() => onSelectRoot()} />
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
      on:primaryAction={onSelectRoot}
      primaryActionText="Back to source"
      style="height: 320px;"
      text="This file doesn't exist on this branch." />
  {/if}
</div>
