<script>
  import { Icon } from "../Primitive";

  export let contributors = null;
  export let branches = null;
  export let commits = null;

  export let style = null;

  const formattedStats = [
    { icon: Icon.Branch, count: branches },
    { icon: Icon.Commit, count: commits },
    { icon: Icon.Member, count: contributors },
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

  .stat p {
    color: var(--color-foreground-level-6);
  }
</style>

<div class="stats" {style}>
  {#each formattedStats as stat}
    <span class="stat">
      <svelte:component this={stat.icon} style="margin-right: 4px;" />
      <p class="typo-mono-bold">{stat.count}</p>
      <slot />
    </span>
  {/each}
</div>
