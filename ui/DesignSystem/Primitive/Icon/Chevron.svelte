<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
  export let variant = "right";
</script>

<IconWrapper on:click {dataCy} {style}>
  {#if variant === 'right'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M9.29289 16.7071C8.90237 16.3166 8.90237 15.6834 9.29289
      15.2929L12.5858 12L9.29289 8.70711C8.90237 8.31658 8.90237 7.68342 9.29289
      7.29289C9.68342 6.90237 10.3166 6.90237 10.7071 7.29289L14.7071
      11.2929C15.0976 11.6834 15.0976 12.3166 14.7071 12.7071L10.7071
      16.7071C10.3166 17.0976 9.68342 17.0976 9.29289 16.7071Z" />
  {:else if variant === 'down'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M16.7071 9.29289C17.0976 9.68342 17.0976 10.3166 16.7071
      10.7071L12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929
      14.7071L7.29289 10.7071C6.90237 10.3166 6.90237 9.68342 7.29289
      9.29289C7.68342 8.90237 8.31658 8.90237 8.70711 9.29289L12 12.5858L15.2929
      9.29289C15.6834 8.90237 16.3166 8.90237 16.7071 9.29289Z" />
  {/if}
</IconWrapper>
