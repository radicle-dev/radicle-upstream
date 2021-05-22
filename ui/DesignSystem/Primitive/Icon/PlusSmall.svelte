<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 6C12.5523 6 13 6.44772 13 7V17C13 17.5523 12.5523 18 12 18C11.4477 18
    11 17.5523 11 17V13H7C6.44772 13 6 12.5523 6 12C6 11.4477 6.44772 11 7
    11H11V7C11 6.44772 11.4477 6 12 6ZM14 12C14 11.4477 14.4477 11 15
    11H17C17.5523 11 18 11.4477 18 12C18 12.5523 17.5523 13 17 13H15C14.4477 13
    14 12.5523 14 12Z" />
</IconWrapper>
