<script>
  import { getContext } from "svelte";
  import { link } from "svelte-spa-router";

  import { Icon } from "../../Primitive";

  import * as path from "../../../lib/path.js";
  import { BLOB } from "../../../../native/types.js";
  import {
    revisionStore,
    objectPathStore
  } from "../../../store/sourceBrowser.js";

  export let name = null;
  export let filePath = null;

  const id = getContext("projectId");

  $: active = filePath === $objectPathStore;
</script>

<style>
  .file {
    display: flex;
    cursor: pointer;
    margin: 0 4px 12px 8px;
    color: var(--color-foreground-level-6);
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
    color: var(--color-secondary);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-secondary);
  }
</style>

<div class="file" class:active>
  <a href={path.projectSource(id, $revisionStore, BLOB, filePath)} use:link>
    <Icon.File />
    {name}
  </a>
</div>
