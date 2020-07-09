<script>
  import { Icon, Title } from "../Primitive";

  export let rad = null;
  export let usd = null;
  export let style = null;
  export let variant = "regular"; // regular | deposit

  let hover = false;
  const enter = () => {
    hover = true;
  };
  const leave = () => {
    hover = false;
  };
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .amount {
    display: flex;
    align-items: center;
    margin-bottom: 4px;
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
</style>

<div class="wrapper" {style} on:mouseenter={enter} on:mouseleave={leave}>
  <div class="amount {variant}">
    {#if variant === 'deposit'}
      <Icon.LockSmall
        style="fill: var(--color-foreground-level-5); margin-right: 2px;" />
      {#if !hover}
        <Icon.Currency style="fill: var(--color-foreground-level-6);" />
      {/if}
    {:else if !hover}
      <Icon.Currency style="fill: var(--color-negative);" />
    {/if}
    <Title variant="tiny" dataCy="amount" style="margin-left:2px;">
      {#if hover}${usd}{:else}{rad}{/if}
    </Title>
  </div>
</div>
