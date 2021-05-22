<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    d="M12 2C12.5523 2 13 2.44772 13 3V4C13 4.55228 12.5523 5 12 5C11.4477 5 11
    4.55228 11 4V3C11 2.44772 11.4477 2 12 2Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M6 12C6 8.68629 8.68629 6 12 6C15.3137 6 18 8.68629 18 12C18 15.3137
    15.3137 18 12 18C8.68629 18 6 15.3137 6 12ZM12 8C9.79086 8 8 9.79086 8 12C8
    14.2091 9.79086 16 12 16C14.2091 16 16 14.2091 16 12C16 9.79086 14.2091 8 12
    8Z" />
  <path
    d="M13 20C13 19.4477 12.5523 19 12 19C11.4477 19 11 19.4477 11 20V21C11
    21.5523 11.4477 22 12 22C12.5523 22 13 21.5523 13 21V20Z" />
</IconWrapper>
