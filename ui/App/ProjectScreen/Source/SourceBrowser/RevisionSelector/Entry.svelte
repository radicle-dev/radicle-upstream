<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { RevisionType } from "ui/src/source";
  import type { Branch, Tag } from "ui/src/source";

  import Badge from "ui/DesignSystem/Badge.svelte";
  import Icon from "ui/DesignSystem/Icon";
  import Spinner from "ui/DesignSystem/Spinner.svelte";

  export let loading: boolean = false;
  export let defaultBranch: boolean = false;
  export let revision: Branch | Tag;
  export let selected: boolean = false;
  export let style: string | undefined = undefined;
</script>

<style>
  .revision-entry {
    align-items: center;
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    height: 2.5rem;
    overflow: hidden;
    overflow-wrap: anywhere;
    user-select: none;
  }

  .revision-entry:hover {
    background: var(--color-foreground-level-1);
  }

  .selected,
  .selected:hover {
    background-color: var(--color-foreground-level-2);
  }

  .revision-name {
    color: var(--color-foreground-level-6);
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }
</style>

<div
  class="revision-entry"
  class:selected
  data-cy={`revision-${revision.type}-${revision.name}`}
  on:click|stopPropagation
  {style}>
  {#if loading}
    <Spinner />
  {:else if revision.type === RevisionType.Branch}
    <Icon.Branch
      dataCy="branch-icon"
      style="vertical-align: bottom; fill: var(--color-foreground-level-4);
      flex-shrink: 0;" />
  {:else if revision.type === RevisionType.Tag}
    <Icon.Label
      dataCy="tag-icon"
      style="vertical-align: bottom; fill: var(--color-foreground-level-4);
    flex-shrink: 0;" />
  {/if}
  <p class="revision-name typo-overflow-ellipsis">{revision.name}</p>
  {#if defaultBranch}
    <Badge variant="default" />
  {/if}
</div>
