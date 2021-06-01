<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M18.5882 6.19127C19.0348 6.51611 19.1336 7.14153 18.8087 7.58818L12.1854
    16.6953C11.1011 18.1861 8.94138 18.3556 7.63786 17.0521L5.29289
    14.7071C4.90237 14.3166 4.90237 13.6834 5.29289 13.2929C5.68342 12.9024
    6.31658 12.9024 6.70711 13.2929L9.05208 15.6379C9.48658 16.0724 10.2065
    16.0159 10.5679 15.5189L17.1913 6.41184C17.5161 5.96519 18.1415 5.86644
    18.5882 6.19127Z" />
</IconWrapper>
