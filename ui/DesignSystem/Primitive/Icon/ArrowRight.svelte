<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M19.7071 11.2929C20.0976 11.6834 20.0976 12.3166 19.7071 12.7071L14.7071
    17.7071C14.3166 18.0976 13.6834 18.0976 13.2929 17.7071C12.9024 17.3166
    12.9024 16.6834 13.2929 16.2929L17.5858 12L13.2929 7.70711C12.9024 7.31658
    12.9024 6.68342 13.2929 6.29289C13.6834 5.90237 14.3166 5.90237 14.7071
    6.29289L19.7071 11.2929ZM14 13L5 13C4.44771 13 4 12.5523 4 12C4 11.4477
    4.44771 11 5 11L14 11C14.5523 11 15 11.4477 15 12C15 12.5523 14.5523 13 14
    13Z" />
</IconWrapper>
