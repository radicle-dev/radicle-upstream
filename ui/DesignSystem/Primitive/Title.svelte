<script>
  export let style = null;

  export let variant = "regular"; // regular | big | huge
</script>

<style>
  .huge {
    font-family: var(--typeface-bold);
    font-size: 36px;
    line-height: 43px;
  }

  .big {
    font-family: var(--typeface-bold);
    font-size: 24px;
    line-height: 29px;
  }

  .regular {
    font-family: var(--typeface-medium);
    font-size: 16px;
    line-height: 130%;
  }
</style>

{#if variant === 'huge'}
  <h1 class={variant} {style}>
    <slot />
  </h1>
{:else if variant === 'big'}
  <h2 class={variant} {style}>
    <slot />
  </h2>
{:else}
  <h3 class={variant} {style}>
    <slot />
  </h3>
{/if}
