<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M10.2971 16.7071C9.90661 16.3166 9.90661 15.6834 10.2971 15.2929L13.59
    12L10.2971 8.70711C9.90661 8.31658 9.90661 7.68342 10.2971 7.29289C10.6877
    6.90237 11.3208 6.90237 11.7113 7.29289L15.0042 10.5858C15.7853 11.3668
    15.7853 12.6332 15.0042 13.4142L11.7113 16.7071C11.3208 17.0976 10.6877
    17.0976 10.2971 16.7071Z" />
</IconWrapper>
