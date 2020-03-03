<script>
  export let style = null;
</script>

<style>
  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 16px;
    line-height: 20px;
  }
</style>

<p class="code" {style}>
  <slot />
</p>
