<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M16.4961 8.13174C16.9757 8.40575 17.1423 9.01661 16.8682 9.49613L13.8426
    14.791C13.0247 16.2222 11.0698 16.484 9.90422 15.3184L8.29289
    13.7071C7.90237 13.3166 7.90237 12.6834 8.29289 12.2929C8.68342 11.9024
    9.31658 11.9024 9.70711 12.2929L11.3184 13.9042C11.5516 14.1373 11.9425
    14.085 12.1061 13.7987L15.1318 8.50385C15.4058 8.02433 16.0166 7.85773
    16.4961 8.13174Z" />
</IconWrapper>
