<script>
  export let style = null;
  export let variant = "regular"; // regular | small
</script>

<svg
  on:click
  {style}
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="var(--color-foreground-level-5)"
  xmlns="http://www.w3.org/2000/svg">
  {#if variant === 'regular'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4
      12.5523 4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13
      19.5523 13 19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19
      11H13V5C13 4.44772 12.5523 4 12 4Z" />
  {:else if variant === 'small'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M12 6C12.5523 6 13 6.44772 13 7V11H17C17.5523 11 18 11.4477 18 12C18
      12.5523 17.5523 13 17 13H13V17C13 17.5523 12.5523 18 12 18C11.4477 18 11
      17.5523 11 17V13H7C6.44772 13 6 12.5523 6 12C6 11.4477 6.44772 11 7
      11H11V7C11 6.44772 11.4477 6 12 6Z" />
  {/if}
</svg>
