<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
  export let variant = "regular"; // regular | small
</script>

<IconWrapper on:click {dataCy} {style}>
  {#if variant === 'regular'}
    <path
      d="M13 5C13 4.44772 12.5523 4 12 4C11.4477 4 11 4.44772 11 5V11L5
      11C4.44772 11 4 11.4477 4 12C4 12.5523 4.44771 13 5 13L11 13V19C11 19.5523
      11.4477 20 12 20C12.5523 20 13 19.5523 13 19V5Z" />
    <path
      d="M19 13C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19
      11H15C14.4477 11 14 11.4477 14 12C14 12.5523 14.4477 13 15 13H19Z" />
  {:else if variant === 'small'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M12 6C12.5523 6 13 6.44772 13 7V17C13 17.5523 12.5523 18 12 18C11.4477
      18 11 17.5523 11 17V13H7C6.44772 13 6 12.5523 6 12C6 11.4477 6.44772 11 7
      11H11V7C11 6.44772 11.4477 6 12 6ZM14 12C14 11.4477 14.4477 11 15
      11H17C17.5523 11 18 11.4477 18 12C18 12.5523 17.5523 13 17 13H15C14.4477
      13 14 12.5523 14 12Z" />
  {/if}
</IconWrapper>
