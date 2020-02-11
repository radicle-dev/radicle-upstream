<script>
  import { Icon, Text } from "../DesignSystem";

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
      <Text
        variant="bigNumber"
        style="display: inline-flex; color: var(--color-darkgray)">
        {amount}
      </Text>
    {:else}
      <Text
        variant="number"
        style="display: inline-flex; color: var(--color-darkgray)">
        {amount}
      </Text>
    {/if}
  </div>
  <Text variant="number" style="color: var(--color-gray)">${amount / 10}</Text>
</div>
