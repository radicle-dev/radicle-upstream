<script>
  import { Icon, Title } from "../../Primitive";

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
</style>

<div {style} class="statusbar {variant}">
  {#if variant === 'negative'}
    <Icon.Important style="margin: 16px; fill: var(--color-background)" />
  {:else if variant === 'positive'}
    <Icon.Check
      variant="filled"
      style="margin: 16px; fill: var(--color-background)" />
  {:else}
    <Icon.TransactionState
      {progress}
      variant="inverted"
      style="margin: 16px; fill: var(--color-background)"
      state={variant} />
  {/if}
  <Title
    variant="medium"
    style="align-self: center; color: var(--color-background);">
    {text}
  </Title>
</div>
