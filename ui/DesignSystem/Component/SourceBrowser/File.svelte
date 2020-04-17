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
    padding: 4px 4px 4px 4px;
    margin: 4px 0;
    color: var(--color-foreground-level-6);
    line-height: 1.5em;
    flex: 1;
    width: 100%;
  }

  /* prevent icon from shrinking when the filename is long */
  .file :global(svg) {
    overflow: unset;
  }

  a {
    display: flex;
    border-radius: 4px;
  }

  a:hover {
    background-color: var(--color-foreground-level-1);
  }

  .active {
    color: var(--color-foreground);
    background-color: var(--color-foreground-level-1);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<a
  class="file"
  class:active
  href={path.projectSource(id, $revisionStore, BLOB, filePath)}
  use:link>
  <Icon.File />
  {name}
</a>
