<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M20 12C20 12.5523 19.5523 13 19 13L5 13C4.44771 13 4 12.5523 4 12C4
    11.4477 4.44771 11 5 11L19 11C19.5523 11 20 11.4477 20 12Z" />
</IconWrapper>
