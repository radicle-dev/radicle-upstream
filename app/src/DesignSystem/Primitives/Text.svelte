<script>
  export let style = null;

  export let variant = "regular"; // regular | small | smallBold
</script>

<style>
  .regular {
    font-family: var(--typeface-regular);
    font-size: 16px;
    line-height: 24px;
  }

  .small {
    font-family: var(--typeface-regular);
    font-size: 14px;
    line-height: 20px;
  }

  .smallBold {
    font-family: var(--typeface-medium);
    font-size: 14px;
    line-height: 20px;
  }
</style>

<p class={variant} {style}>
  <slot />
</p>
