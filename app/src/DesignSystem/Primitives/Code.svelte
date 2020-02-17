<script>
  export let style = null;
</script>

<style>
  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
  }
</style>

<p class="code" {style}>
  <slot />
</p>
