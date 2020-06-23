<script>
  export let style = null;
  export let size = "normal"; // normal | small
</script>

{#if size === 'normal'}
  <svg
    on:click
    {style}
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="var(--color-foreground-level-5)"
    xmlns="http://www.w3.org/2000/svg">
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M8 8V6C8 4.89543 8.89543 4 10 4H18C19.1046 4 20 4.89543 20 6V14C20
      15.1046 19.1046 16 18 16H16V18C16 19.1046 15.1046 20 14 20H6C4.89543 20 4
      19.1046 4 18V10C4 8.89543 4.89543 8 6 8H8ZM10 6H18V14H10V6ZM8 10H6L6
      18H14V16H10C8.89543 16 8 15.1046 8 14V10Z" />
  </svg>
{:else if size === 'small'}
  <svg
    on:click
    {style}
    width="16"
    height="16"
    viewBox="0 0 16 16"
    fill="var(--color-foreground-level-5)"
    xmlns="http://www.w3.org/2000/svg">
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M5 4C5 2.34315 6.34315 1 8 1H12C13.6569 1 15 2.34315 15 4V8C15 9.65685
      13.6569 11 12 11H11V12C11 13.6569 9.65685 15 8 15H4C2.34315 15 1 13.6569 1
      12V8C1 6.34315 2.34315 5 4 5H5V4ZM5 7H4C3.44772 7 3 7.44772 3 8V12C3
      12.5523 3.44772 13 4 13H8C8.55228 13 9 12.5523 9 12V11H8C6.34315 11 5
      9.65685 5 8V7ZM8 3C7.44772 3 7 3.44772 7 4V8C7 8.55228 7.44772 9 8
      9H12C12.5523 9 13 8.55228 13 8V4C13 3.44772 12.5523 3 12 3H8Z" />
  </svg>
{/if}
