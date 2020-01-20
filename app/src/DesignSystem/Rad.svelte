<script>
  import Icon from "../DesignSystem/Icon";
  import Numeric from "../DesignSystem/Numeric";

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
    <Icon.EllipseBig style="fill: var(--color-pink); margin-right: 4px" />
    {#if size === 'big'}
      <Numeric.Big style="display: inline-flex; color: var(--color-darkgray)">
        {amount}
      </Numeric.Big>
    {:else}
      <Numeric.Regular
        style="display: inline-flex; color: var(--color-darkgray)">
        {amount}
      </Numeric.Regular>
    {/if}
  </div>
  <Numeric.Regular style="color: var(--color-gray)">
    ${amount / 10}
  </Numeric.Regular>
</div>
