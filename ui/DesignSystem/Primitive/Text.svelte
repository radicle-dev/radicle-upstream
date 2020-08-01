<script>
  export let style = null;

  export let variant = "regular"; // regular | small | tiny | medium
  export let align = "left"; // left | center | end | justify

  $: textAlign = `text-align: ${align}`;
</script>

<style>
  .regular {
    font-family: var(--typeface-regular);
    font-size: 16px;
    line-height: 24px;
  }

  .small {
    font-family: var(--typeface-medium);
    font-size: 14px;
    line-height: 20px;
  }

  .tiny {
    font-family: var(--typeface-regular);
    font-size: 14px;
    line-height: 20px;
  }
  .medium {
    font-family: var(--typeface-medium);
  }
</style>

<p class={variant} style={style ? style.concat(textAlign) : textAlign}>
  <slot />
</p>
