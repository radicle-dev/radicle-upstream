<script>
  import { Icon } from "../Primitive";
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
    border-radius: 0.25rem;
    padding-right: 0.25rem;
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
        <Icon.CurrencyRADSmall
          style={variant === 'credit' ? 'fill: var(--color-negative);' : 'fill: var(--color-positive);'} />
        <p class="typo-text-small-bold" data-cy="amount">
          {variant === 'credit' ? `-${rad}` : `+${rad}`}
        </p>
      </div>
    {:else if size === 'big'}
      <div class="big">
        <Icon.CurrencyRAD
          style="transform: scale(1.83); fill: var(--color-secondary);" />
        <h1
          style="color: var(--color-secondary); margin-left: 0.25rem;"
          data-cy="amount">
          {rad}
        </h1>
      </div>
    {/if}
  </Tooltip>
</div>
