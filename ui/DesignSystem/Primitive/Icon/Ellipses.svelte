<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
  export let size = "normal"; // normal | small
</script>

{#if size === 'normal'}
  <IconWrapper on:click {dataCy} {style}>
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M5 14C6.10457 14 7 13.1046 7 12C7 10.8954 6.10457 10 5 10C3.89543 10 3
      10.8954 3 12C3 13.1046 3.89543 14 5 14ZM14 12C14 13.1046 13.1046 14 12
      14C10.8954 14 10 13.1046 10 12C10 10.8954 10.8954 10 12 10C13.1046 10 14
      10.8954 14 12ZM21 12C21 13.1046 20.1046 14 19 14C17.8954 14 17 13.1046 17
      12C17 10.8954 17.8954 10 19 10C20.1046 10 21 10.8954 21 12Z" />
  </IconWrapper>
{:else if size === 'small'}
  <IconWrapper on:click {dataCy} {style} width="16" height="16">
    <path
      d="M5 8C5 8.55228 4.55228 9 4 9C3.44772 9 3 8.55228 3 8C3 7.44772 3.44772
      7 4 7C4.55228 7 5 7.44772 5 8Z" />
    <path
      d="M9 8C9 8.55228 8.55228 9 8 9C7.44772 9 7 8.55228 7 8C7 7.44772 7.44772
      7 8 7C8.55228 7 9 7.44772 9 8Z" />
    <path
      d="M13 8C13 8.55228 12.5523 9 12 9C11.4477 9 11 8.55228 11 8C11 7.44772
      11.4477 7 12 7C12.5523 7 13 7.44772 13 8Z" />
  </IconWrapper>
{/if}
