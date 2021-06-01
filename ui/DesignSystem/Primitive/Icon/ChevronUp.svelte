<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M16.7071 13.7029C16.3166 14.0934 15.6834 14.0934 15.2929 13.7029L12
    10.41L8.70711 13.7029C8.31658 14.0934 7.68342 14.0934 7.29289
    13.7029C6.90237 13.3124 6.90237 12.6792 7.29289 12.2887L10.5858
    8.99579C11.3668 8.21474 12.6332 8.21474 13.4142 8.99579L16.7071
    12.2887C17.0976 12.6792 17.0976 13.3124 16.7071 13.7029Z" />
</IconWrapper>
