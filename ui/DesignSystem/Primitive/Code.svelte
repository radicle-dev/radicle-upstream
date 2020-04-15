<script>
  export let style = null;

  export let variant = "regular"; // regular | medium
</script>

<style>
  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 16px;
    line-height: 1.4em;
  }

  .medium {
    font-family: var(--typeface-mono-medium);
  }
</style>

<p class={`code ${variant}`} {style}>
  <slot />
</p>
