<script lang="typescript">
  import { isMarkdown } from "../../../src/source";
  import type { Blob, CommitHeader } from "../../../src/source";

  import { Icon, Markdown } from "../../Primitive";
  import EmptyState from "../EmptyState.svelte";

  import CommitTeaser from "./CommitTeaser.svelte";

  export let blob: Blob;
  export let commit: CommitHeader;
</script>

<style>
  .file-source {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    min-width: var(--content-min-width);
  }

  header .file-header {
    display: flex;
    height: 3rem;
    align-items: center;
    padding-left: 13px;
    color: var(--color-foreground);
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  header .file-name {
    margin-left: 0.5rem;
  }

  header .commit-header {
    height: 2.5rem;
    margin: 0.5rem;
  }

  .line-numbers {
    color: var(--color-foreground-level-4);
    text-align: right;
    user-select: none;
    padding: 0 1rem 0.5rem 1rem;
  }

  .code {
    padding-bottom: 0.5rem;
    overflow-x: auto;
  }

  .container {
    display: flex;
  }

  .markdown-wrapper {
    width: 100%;
    padding: 1rem 2rem;
  }

  .file-source > header + .container .markdown-wrapper {
    padding-top: 0;
  }

  .no-scrollbar {
    scrollbar-width: none;
  }

  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
</style>

<div class="file-source">
  <header>
    <div class="file-header typo-semi-bold" data-cy="file-header">
      <Icon.File />
      <span class="file-name">
        <span>{blob.path.split("/").join(" / ")}</span>
      </span>
    </div>
    <div class="commit-header">
      <CommitTeaser {commit} on:select style="height: 100%" />
    </div>
  </header>
  <div class="container">
    {#if blob.binary}
      <EmptyState
        emoji="👀"
        text="Binary content"
        style="height: 100%; padding: 2rem 0 1rem;" />
    {:else if isMarkdown(blob.path)}
      <div class="markdown-wrapper">
        <Markdown content={blob.content} />
      </div>
    {:else}
      <pre
        class="line-numbers typo-text-mono">
        {@html blob.content
          .split('\n')
          .slice(0, -1)
          .map((_, index) => {
            return `${index + 1}`;
          })
          .join('\n')}
      </pre>
      <pre
        class="code typo-text-mono no-scrollbar">
        {#if blob.html}
          {@html blob.content}
        {:else}{blob.content}{/if}
      </pre>
    {/if}
  </div>
</div>
