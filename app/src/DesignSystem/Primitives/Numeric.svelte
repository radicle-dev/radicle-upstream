<script>
  export let style = null;

  export let variant = "regular"; // small | regular | big
</script>

<style>
  .big {
    font-family: var(--typeface-mono-bold);
    font-size: 22px;
    line-height: 27px;
  }

  .regular {
    font-family: var(--typeface-mono-medium);
    font-size: 16px;
    line-height: 19px;
  }

  .small {
    font-family: var(--typeface-mono-medium);
    font-size: 12px;
    line-height: 14px;
  }
</style>

<p class={variant} {style}>
  <slot />
</p>
