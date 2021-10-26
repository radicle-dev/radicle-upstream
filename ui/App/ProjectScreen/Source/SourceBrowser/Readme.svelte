<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { isMarkdown } from "ui/src/source";

  import FileIcon from "design-system/icons/File.svelte";
  import Markdown from "design-system/Markdown.svelte";

  export let content: string;
  export let path: string;
</script>

<style>
  .readme {
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

  .code {
    padding: 1.5rem;
    overflow-x: auto;
  }

  .container {
    padding: 1rem 2rem;
  }
</style>

<div class="readme">
  <header>
    <div class="file-header typo-text-bold">
      <FileIcon />
      <span class="file-name">{path}</span>
    </div>
  </header>
  <div class="container">
    {#if isMarkdown(path)}
      <Markdown {content} />
    {:else}
      <pre class="code typo-text-mono">{content}</pre>
    {/if}
  </div>
</div>
