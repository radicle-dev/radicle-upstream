<script>
  export let active = false;
  export let style = active && "fill:var(--color-purple)";
</script>

<style>
  svg {
    pointer-events: all;
    cursor: pointer;
  }
  path {
    fill: var(--color-gray);
  }
  svg:hover path {
    fill: var(--color-purple);
  }
</style>

<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
  <path
    {style}
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4 12.5523
    4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13
    19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11H13V5C13
    4.44772 12.5523 4 12 4Z" />
</svg>
