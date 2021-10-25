<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { Icon } from "ui/DesignSystem";

  export let contributors: number;
  export let branches: number;
  export let commits: number;

  export let style: string | undefined = undefined;

  $: formattedStats = [
    { icon: Icon.Branch, count: branches },
    { icon: Icon.Commit, count: commits },
    { icon: Icon.User, count: contributors },
  ];
</script>

<style>
  .stats {
    display: flex;
    flex-direction: row;
  }

  .stat {
    display: flex;
    align-items: center;
    margin-right: 32px;
  }

  .stat:last-child {
    margin-right: 0;
  }

  .stat p {
    color: var(--color-foreground-level-6);
  }
</style>

<div class="stats" {style}>
  {#each formattedStats as stat}
    {#if stat.count > 0}
      <span class="stat">
        <svelte:component this={stat.icon} style="margin-right: 4px;" />
        <p class="typo-text-mono-bold">{stat.count}</p>
        <slot />
      </span>
    {/if}
  {/each}
</div>
