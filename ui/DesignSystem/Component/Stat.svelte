<script>
  import { Numeric } from "../Primitive";

  export let count = null;
  export let icon = null;
  export let style = null;
</script>

<style>
  .stat {
    display: flex;
    align-items: center;
  }
</style>

<div class="stat" {style}>
  <svelte:component this={icon} style="margin-right: 4px;" />
  <Numeric style="color: var(--color-gray);">{count}</Numeric>
</div>
