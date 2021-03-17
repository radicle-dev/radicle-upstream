<script lang="typescript">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../../src/path";
  import type { Project } from "../../../src/project";
  import { store } from "../../../src/screen/project/source";
  import type { MergeRequest } from "../../../src/source";

  import {
    Copyable,
    EmptyState,
    Error,
    Remote,
  } from "../../../DesignSystem/Component";
  import { Button } from "../../../DesignSystem/Primitive";
  import MergeRequestList from "./MergeRequestList.svelte";

  const project: Project = getContext("project-page").project;
  const select = ({ detail: mergeRequest }: { detail: MergeRequest }) => {
    push(path.projectSourceMergeRequest(project.urn, mergeRequest));
  };
  let copyable;

  const copy = () => {
    copyable.copy();
  };
</script>

<Remote {store} let:data={{ mergeRequests }}>
  {#if mergeRequests.length > 0}
    <MergeRequestList
      {mergeRequests}
      defaultBranch={project.metadata.defaultBranch}
      on:select={select} />
  {:else}
    <EmptyState
      emoji="👯‍♀️"
      text="There’s nothing here yet, get started by opening your first merge request.">
      <Copyable bind:this={copyable} showIcon={false}>
        <p
          class="typo-text-small-mono"
          style="text-align: left; color: var(--color-foreground-level-6); overflow-x: scroll; padding: .5rem .5rem .5rem .25rem">
          git tag --annotate merge-request/tag-name
          <br />
          git push --tags rad
        </p>
      </Copyable>
      <Button
        variant="primary"
        style="display: block; margin: 1rem auto 0;"
        on:click={copy}>
        Copy
      </Button>
    </EmptyState>
  {/if}

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
