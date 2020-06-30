<script>
  import { Icon, Numeric } from "../Primitive";

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
