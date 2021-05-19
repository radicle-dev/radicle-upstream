<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    d="M5 14C6.10457 14 7 13.1046 7 12C7 10.8954 6.10457 10 5 10C3.89543 10 3
    10.8954 3 12C3 13.1046 3.89543 14 5 14Z" />
  <path
    d="M14 12C14 13.1046 13.1046 14 12 14C10.8954 14 10 13.1046 10 12C10 10.8954
    10.8954 10 12 10C13.1046 10 14 10.8954 14 12Z" />
  <path
    d="M21 12C21 13.1046 20.1046 14 19 14C17.8954 14 17 13.1046 17 12C17 10.8954
    17.8954 10 19 10C20.1046 10 21 10.8954 21 12Z" />
</IconWrapper>
