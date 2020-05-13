<script>
  import marked from "marked";

  export let content = null;
</script>

<style>
  .markdown :global(h1) {
    color: red;
  }

  .markdown :global(h2) {
    color: green;
  }

  .markdown :global(h3) {
    color: blue;
  }

  .markdown :global(hr) {
    border: 0;
    height: 0;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid rgba(255, 255, 255, 0.3);
  }
</style>

{#if content}
  <div class="markdown">
    {@html marked(content)}
  </div>
{/if}
