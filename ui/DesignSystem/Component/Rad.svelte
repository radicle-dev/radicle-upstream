<script>
  import { Icon, Title } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";

  export let style = null;
  export let rad = null;
  export let usd = null;
  export let size = "regular"; // regular | big
  export let variant = "credit"; // credit | debit
</script>

<style>
  .amount {
    display: flex;
    align-items: center;
    border-radius: 4px;
    padding: 2px 4px;
  }

  .amount.credit {
    fill: var(--color-negative);
    color: var(--color-negative);
    background: var(--color-negative-level-1);
  }

  .amount.debit {
    fill: var(--color-positive);
    color: var(--color-positive);
    background: var(--color-positive-level-1);
  }

  .big {
    display: flex;
    align-items: center;
  }
</style>

<div {style}>
  <Tooltip value={`$${usd}`} position="bottom">
    {#if size === 'regular'}
      <div class="amount {variant}">
        <Icon.Currency
          style={variant === 'credit' ? 'fill: var(--color-negative);' : 'fill: var(--color-positive);'} />
        <Title variant="tiny" dataCy="amount" style="margin-left:2px;">
          {variant === 'credit' ? `-${rad}` : `${rad}`}
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
