<script>
  import { Icon } from "../../Primitive";
  import TransactionSpinner from "./Spinner.svelte";

  export let progress = null; // only applys on variant: caution
  export let style = null;
  export let text = null;
  export let variant = null;
</script>

<style>
  .statusbar {
    border-radius: 4px;
    display: flex;
    height: 56px;
    width: 100%;
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

  h4 {
    align-self: center;
    color: var(--color-background);
  }
</style>

<div {style} class="statusbar {variant}">
  {#if variant === 'negative'}
    <Icon.ExclamationCircle
      style="margin: 16px; fill: var(--color-background)" />
  {:else if variant === 'positive'}
    <Icon.CheckCircle style="margin: 16px; fill: var(--color-background)" />
  {:else}
    <TransactionSpinner
      {progress}
      variant="inverted"
      style="margin: 16px; fill: var(--color-background)"
      state={variant} />
  {/if}
  <h4>{text}</h4>
</div>
