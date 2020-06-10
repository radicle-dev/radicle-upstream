<script>
  import { Icon, Numeric } from "../Primitive";

  export let rad = null;
  export let usd = null;
  export let style = null;
  export let size = "regular"; // regular | big
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
  }
</style>

<div class="wrapper" {style}>
  <div class="rad-amount">
    <Icon.Circle style="fill: var(--color-primary); margin-right: 4px" />
    {#if size === 'big'}
      <Numeric
        dataCy="rad-amount"
        variant="big"
        style="display: inline-flex; color: var(--color-foreground-level-6)">
        {rad}
      </Numeric>
    {:else}
      <Numeric
        dataCy="rad-amount"
        style="display: inline-flex; color: var(--color-foreground-level-6)">
        {rad}
      </Numeric>
    {/if}
  </div>
  {#if usd !== null}
    <Numeric dataCy="usd-amount" style="color: var(--color-foreground-level-5)">
      ${usd}
    </Numeric>
  {/if}
</div>
