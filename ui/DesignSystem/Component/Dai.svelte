<script lang="typescript">
  import { Icon } from "../Primitive";

  export let style = "";
  export let variant = "regular"; // negative
  export let color =
    variant === "regular"
      ? "var(--color-foreground-level-6)"
      : "var(--color-negative)";
</script>

<style>
  .dai {
    display: flex;
    align-items: center;
  }
  .negative {
    padding: 3px 8px;
    border-radius: 3px;

    color: var(--color-negative);
    background-color: var(--color-negative-level-1);
  }

  .symbol {
    margin-right: 5px;
  }

  .negative .symbol {
    content: "-";
    color: var(--color-negative);
  }
</style>

<div class="dai" class:negative={variant === 'negative'} {style}>
  <div class="symbol" />
  <Icon.CurrencyDAI
    style="fill: {color}; padding-top: 3px; width: 20px; height: 20px;" />
  <slot />
</div>
