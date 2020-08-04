<script>
  import { Icon, Markdown } from "../../Primitive";
  import { isMarkdown } from "../../../src/source.ts";

  export let content = null;
  export let path = null;
</script>

<style>
  .file-source {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header .file-header {
    display: flex;
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
    padding: 1.5rem;
    overflow-x: auto;
  }

  .container {
    padding: 24px 24px 16px 24px;
  }
</style>

<div class="file-source" data-cy="file-source">
  <header>
    <div class="file-header semi-bold">
      <Icon.File />
      <span class="file-name">{path}</span>
    </div>
  </header>
  <div class="container">
    {#if isMarkdown(path)}
      <Markdown {content} />
    {:else}
      <pre class="code paragraph-mono">{content}</pre>
    {/if}
  </div>
</div>
