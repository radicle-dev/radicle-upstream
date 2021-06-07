<script lang="typescript">
  import { RevisionType } from "ui/src/source";
  import { BadgeType } from "ui/src/badge";
  import type { Branch, Tag } from "ui/src/source";

  import IconBranch from "ui/DesignSystem/Primitive/Icon/Branch.svelte";
  import IconLabel from "ui/DesignSystem/Primitive/Icon/Label.svelte";
  import Spinner from "ui/DesignSystem/Component/Spinner.svelte";
  import Badge from "ui/DesignSystem/Component/Badge.svelte";

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
    height: 2.25rem;
    margin: 0.125rem;
    overflow: hidden;
    overflow-wrap: anywhere;
    user-select: none;
    border-radius: 0.375rem;
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
    <IconBranch
      dataCy="branch-icon"
      style="vertical-align: bottom; fill: var(--color-foreground-level-4);
      flex-shrink: 0;" />
  {:else if revision.type === RevisionType.Tag}
    <IconLabel
      dataCy="tag-icon"
      style="vertical-align: bottom; fill: var(--color-foreground-level-4);
    flex-shrink: 0;" />
  {/if}
  <p class="revision-name typo-overflow-ellipsis">{revision.name}</p>
  {#if defaultBranch}
    <Badge variant={BadgeType.DefaultBranch} />
  {/if}
</div>
