<script>
  export let style = null;

  export let variant = "table";
</script>

<style>
  .table {
    font-family: var(--typeface-medium);
    font-size: 13px;
    line-height: 16px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }
</style>

<h4 class={variant} {style}>
  <slot />
</h4>
