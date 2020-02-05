<script>
  export let style = null;
  export let size = null;
  export let weight = null;

  // let sizeClass = size === "small" ? "smallSize" : "regularSize";
  // let weightClass = size === "bold" ? "boldWeight" : "regularWeight";
  let textClass = [size, weight].join(" ");
</script>

<style>
  p {
    font-family: "GT America Regular";
    font-size: 16px;
    line-height: 24px;
  }

  p.small {
    font-size: 14px;
    line-height: 20px;
  }

  p.bold {
    font-family: "GT America Medium";
  }
</style>

<p class={textClass} {style}>
  <slot />
</p>
