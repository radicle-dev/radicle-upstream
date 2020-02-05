<script>
  export let style = null;
  export let size = null;

  let sizeClass = size ? size : "regular";
</script>

<style>
  small {
    font-family: "GT America Mono Medium";
    font-size: 12px;
    line-height: 14px;
  }

  .regular {
    font-family: "GT America Mono Medium";
    font-size: 16px;
    line-height: 19px;
  }

  .big {
    font-family: "GT America Mono Bold";
    font-size: 22px;
    line-height: 27px;
  }
</style>

<p class={sizeClass} {style}>
  <slot />
</p>
