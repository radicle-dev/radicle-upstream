<script>
  export let style = null;
  export let dataCy = null;

  export let variant = "regular"; // tiny | small | regular | big
</script>

<style>
  .big {
    font-family: var(--typeface-mono-bold);
    font-size: 22px;
    line-height: 28px;
  }

  .regular {
    font-family: var(--typeface-mono-medium);
    font-size: 16px;
    line-height: 20px;
  }

  .small {
    font-family: var(--typeface-mono-medium);
    font-size: 14px;
    line-height: 18px;
  }
</style>

<p data-cy={dataCy} class={variant} {style}>
  <slot />
</p>
