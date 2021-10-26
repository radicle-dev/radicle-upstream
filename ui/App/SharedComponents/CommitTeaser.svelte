<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { formatCommitTime } from "ui/src/source";
  import type { CommitHeader } from "ui/src/source";

  import * as format from "design-system/lib/format";
  import CommitIcon from "design-system/icons/Commit.svelte";

  export let commit: CommitHeader;
  export let style: string | undefined = undefined;

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
    align-items: center;
    margin-right: 1ex;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .align-right {
    margin-left: auto;
    display: flex;
  }

  .commit-message {
    color: var(--commit-message-color, var(--color-primary));
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    padding: 0 8px 0 4px;
    color: var(--commit-sha-color, var(--color-primary));
    cursor: pointer;
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <CommitIcon style="fill: var(--color-primary)" />
    <span class="commit-sha typo-text-small-mono" on:click={onSelect}>
      {format.shortCommitHash(commit.sha1)}
    </span>
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
