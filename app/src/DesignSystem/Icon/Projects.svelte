<script>
  export let active = false;
  export let style = active && "stroke:var(--color-purple)";
</script>

<style>
  svg {
    pointer-events: all;
    cursor: pointer;
  }
  path {
    stroke: var(--color-gray);
  }
  svg:hover path {
    stroke: var(--color-purple);
  }
</style>

<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
  <path
    {style}
    d="M4 8.61803C4 8.23926 4.214 7.893 4.55279 7.72361L11.5528 4.22361C11.8343
    4.08284 12.1657 4.08284 12.4472 4.22361L19.4472 7.72361C19.786 7.893 20
    8.23926 20 8.61803V15.382C20 15.7607 19.786 16.107 19.4472 16.2764L12.4472
    19.7764C12.1657 19.9172 11.8343 19.9172 11.5528 19.7764L4.55279
    16.2764C4.214 16.107 4 15.7607 4 15.382V8.61803Z"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round" />
  <path
    {style}
    d="M4 8L11.5528 11.7764C11.8343 11.9172 12.1657 11.9172 12.4472 11.7764L20 8"
    stroke-width="1.5"
    stroke-linejoin="round" />
</svg>
