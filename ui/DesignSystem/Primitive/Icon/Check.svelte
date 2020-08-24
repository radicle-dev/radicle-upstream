<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M17.6 7.2C18.0418 7.53137 18.1314 8.15817 17.8 8.6L13.1836
    14.7552C12.0882 16.2157 9.95322 16.3674 8.66229 15.0765L6.29289
    12.7071C5.90237 12.3166 5.90237 11.6834 6.29289 11.2929C6.68342 10.9024
    7.31658 10.9024 7.70711 11.2929L10.0765 13.6623C10.5068 14.0926 11.2185
    14.042 11.5836 13.5552L16.2 7.4C16.5314 6.95817 17.1582 6.86863 17.6 7.2Z" />
</IconWrapper>
