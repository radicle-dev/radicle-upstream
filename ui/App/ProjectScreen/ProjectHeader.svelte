<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Stats } from "ui/src/project";

  import dayjs from "dayjs";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import BranchIcon from "design-system/icons/Branch.svelte";
  import UserIcon from "design-system/icons/User.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let name: string;
  export let urn: string;
  export let description: string | null = null;
  export let stats: Stats;
  export let onClick: (() => void) | undefined = undefined;
  export let latestAnchorTimestamp: number | undefined = undefined;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }

  .description {
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--color-foreground-level-6);
  }

  .stats {
    display: flex;
    margin-top: 0.5rem;
  }

  .stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
  }

  .stat-item:not(:last-of-type)::after {
    content: "•";
    display: flex;
    color: var(--color-foreground-level-3);
    margin: 0 1rem 0 1rem;
    font-family: var(--typeface-mono-bold);
    font-weight: bold;
  }

  .stat-item span {
    margin-left: 0.5rem;
    white-space: nowrap;
  }

  .clickable {
    cursor: pointer;
  }
</style>

<div class="metadata">
  <h1
    data-cy="entity-name"
    class="typo-overflow-ellipsis"
    title={name}
    class:clickable={onClick !== undefined}
    on:click={() => (onClick ? onClick() : {})}>
    {name}
  </h1>
  <CopyableIdentifier
    value={urn}
    kind="projectUrn"
    style="margin-top: 0.5rem;" />
  {#if description}
    <p class="description typo-overflow-ellipsis" title={description}>
      {description}
    </p>
  {/if}
  <div class="stats" data-cy="project-stats">
    {#if stats.branches > 0}
      <div class="stat-item">
        <BranchIcon />
        <span>
          {stats.branches === 1 ? `1 Branch` : `${stats.branches} Branches`}
        </span>
      </div>
    {/if}

    {#if stats.contributors > 0}
      <div class="stat-item">
        <UserIcon />
        <span>
          {stats.contributors === 1
            ? `1 Contributor`
            : `${stats.contributors} Contributors`}
        </span>
      </div>
    {/if}

    {#if latestAnchorTimestamp}
      <div class="stat-item">
        <AnchorIcon />
        <span>
          Last anchored {dayjs().to(dayjs.unix(latestAnchorTimestamp))}
        </span>
      </div>
    {/if}
  </div>
</div>
