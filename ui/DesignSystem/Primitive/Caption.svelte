<script>
  export let style = null;
</script>

<style>
  .caption {
    font-family: var(--typeface-medium);
    font-size: 13px;
    line-height: 16px;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }
</style>

<h4 class="caption" {style}>
  <slot />
</h4>
