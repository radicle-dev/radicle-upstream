<script>
  import { Icon, Numeric } from "../Primitive";

  export let rad = null;
  export let usd = null;
  export let style = null;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .rad-amount {
    display: flex;
    align-items: center;
    margin-bottom: 4px;
    display: inline-flex;
    color: var(--color-negative);
    background: var(--color-negative-level-1);
    padding: 2px 4px;
    border-radius: 4px;
  }
</style>

<div class="wrapper" {style}>
  <div class="rad-amount">
    <Icon.Currency style="fill: var(--color-negative); margin: 2px 4px 0 0" />
    <Numeric dataCy="rad-amount">{rad}</Numeric>
  </div>
  {#if usd !== null}
    <Numeric dataCy="usd-amount" style="display:none;">${usd}</Numeric>
  {/if}
</div>
