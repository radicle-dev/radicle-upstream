<script>
  export let style = null;
</script>

<style>
  p {
    font-family: "GT America Medium";
    font-size: 13px;
    line-height: 16px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }
</style>

<p {style}>
  <slot />
</p>
