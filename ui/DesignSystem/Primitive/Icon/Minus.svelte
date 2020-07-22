<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
</script>

<IconWrapper on:click {dataCy} {style}>
  <rect x="4" y="11" width="16" height="2" rx="1" />
</IconWrapper>
