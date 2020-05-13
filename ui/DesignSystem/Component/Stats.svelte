<script>
  import { Icon, Numeric } from "../Primitive";

  export let stats = [
    { icon: Icon.Commit, count: -1 },
    { icon: Icon.Branch, count: -1 },
    { icon: Icon.Member, count: -1 },
  ];

  export let style = null;
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
  {#each stats as stat}
    <span class="stat">
      <svelte:component this={stat.icon} style="margin-right: 4px;" />
      <Numeric style="color: var(--color-foreground-level-6);">
        {stat.count}
      </Numeric>
      <slot />
    </span>
  {/each}
</div>
