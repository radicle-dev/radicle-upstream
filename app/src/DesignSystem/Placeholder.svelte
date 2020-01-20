<script>
  export let style = null;
</script>

<style>
  rect,
  path {
    fill: none;
    stroke: var(--color-lightgray);
    stroke-width: 1px;
    vector-effect: non-scaling-stroke;
    shape-rendering: geometricPrecision;
  }
  rect {
    stroke-width: 2px;
  }
</style>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewbox="0 0 40 40"
  preserveAspectRatio="none"
  {style}>
  <path d="M 20,20 L 40,40" />
  <path d="M 20,20 L 00,40" />
  <path d="M 20,20 L 40,0" />
  <path d="M 20,20 L 0,0" />
  <rect width="40" height="40" />
</svg>
