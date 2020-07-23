<script>
  import { Icon, Title } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";

  export let style = null;
  export let rad = null;
  export let usd = null;
  export let size = "regular"; // regular | big
</script>

<style>
  .amount {
    display: flex;
    align-items: center;
    fill: var(--color-negative);
    color: var(--color-negative);
    background: var(--color-negative-level-1);
    padding: 2px 4px;
    border-radius: 4px;
  }

  .big {
    display: flex;
    align-items: center;
  }
</style>

<div {style}>
  <Tooltip value={`$${usd}`} position="bottom">
    {#if size === 'regular'}
      <div class="amount">
        <Icon.Currency style="fill: var(--color-negative);" />
        <Title variant="tiny" dataCy="amount" style="margin-left:2px;">
          -{rad}
        </Title>
      </div>
    {:else if size === 'big'}
      <div class="big">
        <Icon.Currency size="huge" style="fill: var(--color-secondary);" />
        <Title
          style="color: var(--color-secondary); margin-left: 2px;"
          variant="huge"
          dataCy="amount">
          {rad}
        </Title>
      </div>
    {/if}
  </Tooltip>
</div>
