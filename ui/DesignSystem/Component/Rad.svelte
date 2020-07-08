<script>
  import { Icon, Title } from "../Primitive";

  export let rad = null;
  export let usd = null;
  export let style = null;
  export let deposit = null;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .amount {
    display: flex;
    align-items: center;
    margin-bottom: 4px;
    display: inline-flex;
    fill: var(--color-negative);
    color: var(--color-negative);
    background: var(--color-negative-level-1);
    padding: 2px 4px;
    border-radius: 4px;
  }

  .deposit {
    fill: var(--color-foreground-level-6);
    color: var(--color-foreground-level-6);
    background: var(--color-foreground-level-2);
  }

  .wrapper:hover .rad {
    display: none;
  }

  .usd {
    display: none;
  }

  .wrapper:hover .usd {
    display: flex;
  }
</style>

<div class="wrapper" {style}>
  <div class="amount rad" class:deposit>
    {#if deposit}
      <Icon.LockSmall
        style="fill: var(--color-foreground-level-5); margin-right: 2px;" />
      <Icon.Currency style="fill: var(--color-foreground-level-6);" />
    {:else}
      <Icon.Currency style="fill: var(--color-negative);" />
    {/if}
    <Title variant="tiny" dataCy="rad-amount" style="margin-left:2px;">
      {rad}
    </Title>
  </div>
  {#if usd !== null}
    <div class="amount usd" class:deposit>
      {#if deposit}
        <Icon.LockSmall
          style="fill: var(--color-foreground-level-5); margin-right: 2px;" />
      {/if}
      <Title variant="tiny" dataCy="usd-amount">${usd}</Title>
    </div>
  {/if}
</div>
