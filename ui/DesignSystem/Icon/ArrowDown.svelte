<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12.7071 19.7071C12.3166 20.0976 11.6834 20.0976 11.2929 19.7071L6.29289
    14.7071C5.90237 14.3166 5.90237 13.6834 6.29289 13.2929C6.68342 12.9024
    7.31658 12.9024 7.70711 13.2929L12 17.5858L16.2929 13.2929C16.6834 12.9024
    17.3166 12.9024 17.7071 13.2929C18.0976 13.6834 18.0976 14.3166 17.7071
    14.7071L12.7071 19.7071ZM11 14V5C11 4.44771 11.4477 4 12 4C12.5523 4 13
    4.44771 13 5V14C13 14.5523 12.5523 15 12 15C11.4477 15 11 14.5523 11 14Z" />
</IconWrapper>
