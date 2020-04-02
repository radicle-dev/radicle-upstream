<script>
  import { Icon, Text } from "../../Primitive";

  export let style = null;
  export let variant = "caution"; // caution | negative | positive
  export let progress = 0; // only applys on variant: caution

  const text = {
    caution:
      progress === 0
        ? "Waiting for confirmation"
        : "Waiting for transaction to settle",
    negative: "Transaction failed at 2:22am Today",
    positive: "Transaction settled at 2:22am Today"
  };
</script>

<style>
  .statusbar {
    height: 48px;
    width: 100%;
    border-radius: 4px;
    display: flex;
  }

  .caution {
    background-color: var(--color-caution);
  }

  .negative {
    background-color: var(--color-negative);
  }

  .positive {
    background-color: var(--color-positive);
  }
</style>

<div {style} class="statusbar {variant}">
  {#if variant === 'negative'}
    <Icon.Important style="margin: 12px; fill: var(--color-background)" />
  {:else if variant === 'positive'}
    <Icon.CheckCircle style="margin: 12px; fill: var(--color-background)" />
  {:else}
    <Icon.TxState
      variant="inverted"
      {progress}
      style="margin: 12px; fill: var(--color-background)"
      state={variant} />
  {/if}
  <Text variant="tiny" style="align-self: center;">{text[variant]}</Text>
</div>
