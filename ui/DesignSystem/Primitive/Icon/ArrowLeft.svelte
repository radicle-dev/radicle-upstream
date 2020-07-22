<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M4.29289 12.7071C3.90237 12.3166 3.90237 11.6834 4.29289 11.2929L9.29289
    6.29289C9.68342 5.90237 10.3166 5.90237 10.7071 6.29289C11.0976 6.68342
    11.0976 7.31658 10.7071 7.70711L7.41421 11L19 11C19.5523 11 20 11.4477 20
    12C20 12.5523 19.5523 13 19 13L7.41421 13L10.7071 16.2929C11.0976 16.6834
    11.0976 17.3166 10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289
    17.7071L4.29289 12.7071Z" />
</IconWrapper>
