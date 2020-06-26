<script>
  import { link } from "svelte-spa-router";

  import * as path from "../../../src/path.ts";
  import { ObjectType } from "../../../src/source.ts";

  import { Icon } from "../../Primitive";

  export let projectId = null;
  export let filePath = null;
  export let name = null;
  export let currentRevision = null;
  export let active = false;
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

  .file-name {
    margin-left: 0.25rem;
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
  href={path.projectSource(projectId, currentRevision, ObjectType.Blob, filePath)}
  use:link>
  <Icon.File />
  <span class="file-name">{name}</span>
</a>
