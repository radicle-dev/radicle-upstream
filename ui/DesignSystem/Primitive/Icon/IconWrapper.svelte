<script>
  export let dataCy = null;
  export let style = null;
  export let height = "24";
  export let width = "24";
</script>

<svg
  data-cy={dataCy}
  on:click
  style={`flex-shrink: 0;${style ? style : ''}`}
  {width}
  {height}
  fill="var(--color-foreground-level-5)"
  viewBox={`0 0 ${width} ${height}`}
  xmlns="http://www.w3.org/2000/svg">
  <slot />
</svg>
