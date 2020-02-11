<script>
  import { getContext } from "svelte";
  import { revision, objectPath } from "../../stores.js";
  import { BLOB } from "../../types.js";

  import { Icon } from "../Primitives";
  import { link } from "svelte-spa-router";
  import * as path from "../../path.js";

  export let name = null;
  export let filePath = null;

  const id = getContext("projectId");

  $: active = filePath === $objectPath;
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

  .active a {
    color: var(--color-purple);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-purple);
  }
</style>

<div class="file" class:active>
  <a href={path.projectSource(id, $revision, BLOB, filePath)} use:link>
    <Icon.File />
    {name}
  </a>
</div>
