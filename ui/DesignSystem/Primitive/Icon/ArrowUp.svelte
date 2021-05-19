<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M11.2929 4.29289C11.6834 3.90237 12.3166 3.90237 12.7071 4.29289L17.7071
    9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071C17.3166 11.0976
    16.6834 11.0976 16.2929 10.7071L12 6.41421L7.70711 10.7071C7.31658 11.0976
    6.68342 11.0976 6.29289 10.7071C5.90237 10.3166 5.90237 9.68342 6.29289
    9.29289L11.2929 4.29289ZM13 10V19C13 19.5523 12.5523 20 12 20C11.4477 20 11
    19.5523 11 19V10C11 9.44772 11.4477 9 12 9C12.5523 9 13 9.44772 13 10Z" />
</IconWrapper>
