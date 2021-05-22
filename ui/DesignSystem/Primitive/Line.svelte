<script>
  export let style = null;
  export let height = null;
  export let width = null;
  // Default: vertically centered horizontal line of full width with
  // stroke-width 1 and color gray
  export let x1 = 0;
  export let y1 = height / 2;
  export let x2 = width;
  export let y2 = height / 2;
</script>

<svg
  style="stroke: var(--color-foreground-level-5); stroke-width: 1"
  {height}
  {width}>
  <line {style} {x1} {y1} {x2} {y2} />
</svg>
