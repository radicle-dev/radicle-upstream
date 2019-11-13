<script>
  import { Icon } from "../DesignSystem";
  import { link, location } from "svelte-spa-router";

  export let name = null;
  export let path = null;
</script>

<style>
  .file {
    display: flex;
    cursor: pointer;
    margin: 0 4px 12px 8px;
    color: var(--color-darkgray);
    flex: 1;
  }

  /* prevent icon from shrinking when the filename is long */
  .file :global(svg) {
    overflow: unset;
  }

  a {
    display: flex;
  }
</style>

<div class="file">
  <a href={$location + path} use:link>
    <Icon.File />
    {name}
  </a>
</div>
