<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Stats } from "ui/src/project";

  import { Icon, RadicleId } from "ui/DesignSystem";

  export let name: string;
  export let urn: string;
  export let description: string = "";
  export let stats: Stats;
  export let onClick: (() => void) | undefined = undefined;
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
    margin-right: 1rem;
  }

  .stat-item p {
    margin-left: 0.5rem;
    white-space: nowrap;
  }

  .stat-separator {
    display: flex;
    color: var(--color-foreground-level-3);
    margin-right: 1rem;
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
  <RadicleId truncate {urn} style="margin-top: 0.5rem;" />
  {#if description.length > 0}
    <p class="description typo-overflow-ellipsis" title={description}>
      {description}
    </p>
  {/if}
  <div class="stats" data-cy="project-stats">
    {#if stats.branches > 0}
      <div class="stat-item">
        <Icon.Branch />
        <p>
          {stats.branches === 1 ? `1 Branch` : `${stats.branches} Branches`}
        </p>
      </div>
    {/if}
    {#if stats.branches > 0 && stats.contributors > 0}
      <span class="typo-mono-bold stat-separator">•</span>
    {/if}
    {#if stats.contributors > 0}
      <div class="stat-item">
        <Icon.User />
        <p>
          {stats.contributors === 1
            ? `1 Contributor`
            : `${stats.contributors} Contributors`}
        </p>
      </div>
    {/if}
  </div>
</div>
