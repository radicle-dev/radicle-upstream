<script>
  import { Icon, Numeric } from "../Primitive";

  export let amount = null;
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
        variant="big"
        style="display: inline-flex; color: var(--color-foreground-level-6)">
        {amount}
      </Numeric>
    {:else}
      <Numeric
        style="display: inline-flex; color: var(--color-foreground-level-6)">
        {amount}
      </Numeric>
    {/if}
  </div>
  <Numeric style="color: var(--color-foreground-level-5)">
    ${amount / 10}
  </Numeric>
</div>
