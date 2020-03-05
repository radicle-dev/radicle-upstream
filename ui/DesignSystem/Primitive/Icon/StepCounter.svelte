<script>
  export let style = null;
  export let active = false;
</script>

<svg
  {style}
  width="15"
  height="15"
  viewBox="0 0 15 15"
  fill="none"
  xmlns="http://www.w3.org/2000/svg">
  <path
    d="M7.5 14.5C11.366 14.5 14.5 11.366 14.5 7.5C14.5 3.63401 11.366 0.5 7.5
    0.5C3.63401 0.5 0.5 3.63401 0.5 7.5C0.5 11.366 3.63401 14.5 7.5 14.5Z"
    stroke="var(--color-gray)" />
  {#if active}
    <path
      d="M7.5 12C9.98528 12 12 9.98528 12 7.5C12 5.01472 9.98528 3 7.5 3C5.01472
      3 3 5.01472 3 7.5C3 9.98528 5.01472 12 7.5 12Z"
      fill="var(--color-radblue)" />
  {/if}
</svg>
