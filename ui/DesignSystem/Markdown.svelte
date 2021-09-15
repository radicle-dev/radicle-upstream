<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import marked from "marked";

  export let content: string;

  const renderer = {
    link(href: string, _title: string, text: string) {
      if (
        href.toLowerCase().startsWith("http://") ||
        href.toLowerCase().startsWith("https://")
      ) {
        if (text.includes("<img")) {
          return `<a href="${href}">${text}</a>`;
        } else {
          return `<a class="typo-link" style="text-decoration: none;" href="${href}"><span style="text-decoration: underline; margin-right: 0.1rem;">${text}</span><span style="vertical-align: text-top">↗</span></a>`;
        }
      } else {
        // Internal links don't work yet, so we disable user-interaction.
        return `<span style="text-decoration: underline; text-underline-offset: 0.25rem;">${text}</span>`;
      }
    },
  };

  marked.use({ renderer });
</script>

<style>
  .markdown :global(h1) {
    font-family: var(--typeface-medium);
    font-size: 2rem;
    padding: 1rem 0;
    margin: 0 0 1.75rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  .markdown :global(h1:not(:first-child)) {
    margin-top: 2rem;
  }

  .markdown :global(h1 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 2rem;
    padding: 0.25rem;
  }

  .markdown :global(h2) {
    font-family: var(--typeface-medium);
    font-size: 1.5rem;
    padding: 0.75rem 0;
    margin: 1.8rem 0 1.65rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(h2 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 1.5rem;
    padding: 0.25rem;
  }

  .markdown :global(h3) {
    font-family: var(--typeface-medium);
    font-size: 1.25rem;
    padding: 0.65rem 0;
    margin: 1.75rem 0 1.5rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(h3 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 1.25rem;
    padding: 0.25rem;
  }
  .markdown :global(h4) {
    font-family: var(--typeface-medium);
    font-size: 1rem;
    padding: 0.5rem 0;
    margin: 1.5rem 0 1.35rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(h4 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 1rem;
    padding: 0.25rem;
  }

  .markdown :global(h5) {
    font-family: var(--typeface-medium);
    font-size: 0.875rem;
    padding: 0.35rem 0;
    margin: 1.35rem 0 1.25rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(h5 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 0.875rem;
    padding: 0.25rem;
  }

  .markdown :global(h6) {
    font-family: var(--typeface-medium);
    font-size: 0.75rem;
    padding: 0.25rem 0;
    margin: 1.25rem 0 1rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(h6 > code) {
    font-family: var(--typeface-mono-bold), monospace;
    font-size: 0.75rem;
    padding: 0.25rem;
  }

  .markdown :global(p) {
    margin-top: 0;
    margin-bottom: 0.625rem;
  }

  .markdown :global(strong) {
    font-family: var(--typeface-medium);
  }

  .markdown :global(img) {
    border-style: none;
    max-width: 100%;
  }

  .markdown :global(code) {
    font-family: var(--typeface-mono-regular), monospace;
    font-size: 1rem;
    background-color: var(--color-foreground-level-1);
    padding: 0.09rem 0.25rem;
    border-radius: 0.5rem;
  }
  .markdown :global(pre) {
    font-family: var(--typeface-mono-regular), monospace;
    font-size: 1rem;
    background-color: var(--color-foreground-level-1);
    padding: 1rem;
    border-radius: 0.5rem;
    margin: 1rem 0;
    overflow: scroll;
    scrollbar-width: none;
  }

  .markdown :global(pre::-webkit-scrollbar) {
    display: none;
  }

  .markdown :global(hr) {
    height: 0;
    margin: 0rem 0;
    overflow: hidden;
    background: transparent;
    border: 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .markdown :global(ol) {
    list-style-type: decimal;
    margin-bottom: 1rem;
    padding-left: 1.5rem;
  }

  .markdown :global(ul) {
    list-style-type: inherit;
    padding-left: 1.25rem;
    margin-bottom: 1rem;
  }
</style>

{#if content}
  <div class="markdown">
    {@html marked(content)}
  </div>
{/if}
