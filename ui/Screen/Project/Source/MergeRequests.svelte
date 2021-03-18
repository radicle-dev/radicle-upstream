<script lang="typescript">
  import {
    selectMergeRequest,
    store,
  } from "../../../src/screen/project/source";
  import type { MergeRequest } from "../../../src/source";

  import {
    Copyable,
    EmptyState,
    Error,
    Remote,
  } from "../../../DesignSystem/Component";
  import { Button } from "../../../DesignSystem/Primitive";
  import MergeRequestList from "./MergeRequestList.svelte";

  const onSelect = ({ detail: mergeRequest }: { detail: MergeRequest }) => {
    selectMergeRequest(mergeRequest);
  };
  let copyable: string;

  const copy = () => {
    copyable.copy();
  };

  const instructions = `git tag --annotate merge-request/tag-name && git push --tags rad`;
</script>

<Remote {store} let:data={{ mergeRequests, project }}>
  {#if mergeRequests.length > 0}
    <MergeRequestList
      {mergeRequests}
      defaultBranch={project.metadata.defaultBranch}
      on:select={onSelect} />
  {:else}
    <EmptyState
      emoji="👯‍♀️"
      text="There’s nothing here yet, get started by opening your first merge request.">
      <Copyable bind:this={copyable} showIcon={false}>
        <p
          class="typo-text-small-mono"
          style="text-align: left; color: var(--color-foreground-level-6); overflow-x: scroll; padding: .5rem .5rem .5rem .25rem">
          {instructions}
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
