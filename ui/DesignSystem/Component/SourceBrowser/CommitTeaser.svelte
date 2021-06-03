<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { formatCommitTime } from "../../../src/source";
  import type { CommitHeader } from "../../../src/source";

  import { Icon } from "../../Primitive";

  export let commit: CommitHeader;
  export let style: string = "";

  const dispatch = createEventDispatcher();
  const onSelect = () => {
    dispatch("select", commit.sha1);
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0 12px 0 8px;
    white-space: nowrap;
    min-width: var(--content-min-width);
    border-radius: 0.5rem;
    background-color: var(--color-primary-level-1);
  }

  .align-left {
    display: flex;
    flex: 1;
    align-items: center;
    max-width: 64%;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }

  .commit-message {
    color: var(--commit-message-color, var(--color-primary));
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    padding: 0 8px 0 4px;
    color: var(--commit-sha-color, var(--color-primary));
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <Icon.Commit style="fill: var(--color-primary)" />
    <!-- svelte-ignore a11y-missing-attribute -->
    <a class="commit-sha typo-text-small-mono" on:click={onSelect}>
      {commit.sha1.substring(0, 7)}
    </a>
    <p class="commit-message typo-text-small">{commit.summary}</p>
  </div>

  <div class="align-right">
    <p
      class="typo-text-small-bold"
      style="margin-right: 8px; color: var(--color-foreground-level-6)">
      {commit.author.name}
    </p>
    <p class="typo-text-small" style="color: var(--color-foreground-level-6)">
      {formatCommitTime(commit.committerTime)}
    </p>
  </div>
</div>
