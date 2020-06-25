<script>
  import { Icon, Numeric } from "../Primitive";

  export let stats = {
    contributors: -1,
    branches: -1,
    commits: -1,
  };

  export let style = null;

  const formattedStats = [
    { icon: Icon.Branch, count: stats.branches },
    { icon: Icon.Commit, count: stats.commits },
    { icon: Icon.Member, count: stats.contributors },
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
</style>

<div class="stats" {style}>
  {#each formattedStats as stat}
    <span class="stat">
      <svelte:component this={stat.icon} style="margin-right: 4px;" />
      <Numeric style="color: var(--color-foreground-level-6);">
        {stat.count}
      </Numeric>
      <slot />
    </span>
  {/each}
</div>
