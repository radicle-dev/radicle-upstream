<script lang="typescript">
  import { Icon } from "../Primitive";

  export let style = "";
  export let color = "var(--color-foreground-level-6)";
</script>

<style>
  .dai {
    display: flex;
    align-items: center;
  }
</style>

<div class="dai" {style}>
  <Icon.CurrencyDAI
    style="fill: {color}; padding-top: 3px; width: 20px; height: 20px;" />
  <slot />
</div>
