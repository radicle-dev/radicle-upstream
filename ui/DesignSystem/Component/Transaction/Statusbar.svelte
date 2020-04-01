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
  <!-- TODO(merle): Add inverted color icons -->
  <Icon.TxState
    style="margin: 12px; background-color: var(--color-background)"
    state={variant} />
  <Text variant="tiny" style="align-self: center;">{text[variant]}</Text>
</div>
