<script>
  import { Icon } from "../../Primitive";

  export let blob = null;
  export let path = null;

  console.assert(blob.binary === false, "blob should not be binary");
</script>

<style>
  .file-source {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header .file-header {
    display: flex;
    font-family: var(--typeface-medium);
    font-size: 1rem;
    height: 3rem;
    align-items: center;
    padding-left: 13px;
    color: var(--color-foreground);
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  header .file-name {
    margin-left: 0.5rem;
  }

  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 16px;
    padding: 1.5rem;
    overflow-x: auto;
  }

  .container {
    display: flex;
  }
</style>

<div class="file-source" data-cy="file-source">
  <header>
    <div class="file-header">
      <Icon.File />
      <span class="file-name">{path}</span>
    </div>
  </header>
  <div class="container">
    <pre class="code">{blob.content}</pre>
  </div>
</div>
