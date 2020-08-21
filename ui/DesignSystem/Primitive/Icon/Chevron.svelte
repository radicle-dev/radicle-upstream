<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
  export let variant = "right";
</script>

<IconWrapper on:click {dataCy} {style} width="16" height="16">
  {#if variant === 'right'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M5.29293 12.7071C4.90243 12.3166 4.90243 11.6834 5.29293
      11.2929L8.58582 8L5.29293 4.7071C4.90243 4.3166 4.90243 3.6834 5.29293
      3.2929C5.68343 2.9024 6.31661 2.9024 6.70714 3.2929L10 6.58579C10.7811
      7.36683 10.7811 8.63316 10 9.41421L6.70714 12.7071C6.31661 13.0976 5.68343
      13.0976 5.29293 12.7071Z" />
  {:else if variant === 'down'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M3.29289 5.2929C3.68342 4.9024 4.31658 4.9024 4.70711 5.2929L8
      8.58579L11.2929 5.2929C11.6834 4.9024 12.3166 4.9024 12.7071
      5.2929C13.0976 5.6834 13.0976 6.31658 12.7071 6.70711L9.41421 10C8.63317
      10.781 7.36684 10.781 6.58579 10L3.29289 6.70711C2.90237 6.31658 2.90237
      5.6834 3.29289 5.2929Z" />
  {/if}
</IconWrapper>
