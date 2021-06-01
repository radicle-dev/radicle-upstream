<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7.29289 10.2971C7.68342 9.90658 8.31658 9.90658 8.70711 10.2971L12
    13.59L15.2929 10.2971C15.6834 9.90658 16.3166 9.90658 16.7071
    10.2971C17.0976 10.6876 17.0976 11.3208 16.7071 11.7113L13.4142
    15.0042C12.6332 15.7853 11.3668 15.7853 10.5858 15.0042L7.29289
    11.7113C6.90237 11.3208 6.90237 10.6876 7.29289 10.2971Z" />
</IconWrapper>
