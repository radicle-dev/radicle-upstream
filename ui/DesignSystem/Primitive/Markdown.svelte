<script>
  import marked from "marked";

  export let content = null;
</script>

<style>
  .markdown :global(h1) {
    font-family: var(--typeface-medium);
    font-size: 2em;
    margin: 0.75em 0;
  }

  .markdown :global(h2) {
    font-family: var(--typeface-medium);
    font-size: 1.5em;
    margin: 0.65em 0;
  }

  .markdown :global(h3) {
    font-family: var(--typeface-medium);
    font-size: 1.25em;
    margin: 0.5em 0;
  }
  .markdown :global(h4) {
    font-family: var(--typeface-medium);
    font-size: 1em;
    margin: 0.35em 0;
  }
  .markdown :global(h5) {
    font-family: var(--typeface-medium);
    font-size: 0.875em;
    margin: 0.25em 0;
  }
  .markdown :global(h6) {
    font-family: var(--typeface-medium);
    font-size: 0.75em;
    margin: 0;
  }

  .markdown :global(p) {
    margin-top: 0;
    margin-bottom: 0.625em;
  }

  .markdown :global(a) {
    background-color: initial;
    color: var(--color-secondary);
    text-decoration: none;
  }
  .markdown :global(a:hover) {
    text-decoration: underline;
  }

  .markdown :global(strong) {
    font-family: var(--typeface-medium);
  }

  .markdown :global(img) {
    border-style: none;
  }

  .markdown :global(code > pre) {
    font-family: var(--typeface-mono-regular), monospace;
    font-size: 1em;
    background-color: var(--color-foreground-level-1);
    padding: 0 1em;
    border-radius: 0.25em;
  }
  .markdown :global(code > pre:first-child) {
    padding-top: 1em;
  }
  .markdown :global(code > pre:last-child) {
    padding-bottom: 1em;
  }

  .markdown :global(hr) {
    height: 0;
    margin: 15px 0;
    overflow: hidden;
    background: transparent;
    border: 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(ol) {
    list-style-type: decimal;
    list-style-position: inside;
    margin-bottom: 1em;
  }

  .markdown :global(ul) {
    list-style-type: inherit;
    padding-left: 1.2em;
    margin-bottom: 1em;
  }
  .markdown :global(li > p) {
    margin: 0;
  }
</style>

{#if content}
  <div class="markdown">
    {@html marked(content)}
  </div>
{/if}
