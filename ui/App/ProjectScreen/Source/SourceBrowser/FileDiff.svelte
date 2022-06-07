<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type {
    ModifiedFile,
    CreatedFile,
    DeletedFile,
  } from "ui/src/source/diff";
  import { lineNumberL, lineNumberR, lineSign } from "ui/src/source/diff";

  import ChevronDownIcon from "design-system/icons/ChevronDown.svelte";
  import ChevronRightIcon from "design-system/icons/ChevronRight.svelte";
  import FileIcon from "design-system/icons/File.svelte";

  export let file: ModifiedFile | CreatedFile | DeletedFile;
  export let label: "created" | "deleted" | undefined = undefined;

  let collapsed: boolean = false;
</script>

<style>
  /* TODO(cloudhead): Reconcile with `FileSource`? */
  .changeset-file {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    min-width: var(--content-min-width);
    margin-bottom: 2rem;
  }
  .changeset-file header {
    cursor: pointer;
    height: 3rem;
    display: flex;
    align-items: center;
    background: none;
    border-radius: 0;
    padding: 0.75rem;
  }
  main {
    border-top: 1px solid var(--color-foreground-level-3);
  }
  .changeset-file main {
    overflow-x: auto;
  }
  .binary {
    padding: 1rem;
    color: var(--color-foreground-level-4);
    text-align: center;
    background-color: var(--color-foreground-level-1);
  }
  table.diff {
    table-layout: fixed;
    border-collapse: collapse;
    margin: 0.5rem 0;
  }
  tr.diff-line[data-type="+"] > * {
    background: var(--color-positive-level-1);
  }
  tr.diff-line[data-type="-"] > * {
    background: var(--color-negative-level-1);
  }
  td.diff-line-number {
    text-align: right;
    user-select: none;
    line-height: 150%;
  }
  td.diff-line-number[data-type="+"],
  td.diff-line-type[data-type="+"] {
    color: var(--color-positive-level-6);
  }
  td.diff-line-number[data-type="-"],
  td.diff-line-type[data-type="-"] {
    color: var(--color-negative-level-6);
  }
  td.diff-line-number.left {
    padding: 0 0.25rem 0 1rem;
  }
  td.diff-line-number.right {
    padding: 0 1rem 0 0.25rem;
  }
  td.diff-line-content {
    white-space: pre;
    width: 100%;
    padding-right: 0.5rem;
  }
  td.diff-line-type {
    user-select: none;
    padding-right: 1rem;
    text-align: center;
  }
  td.diff-expand-action {
    text-align: center;
    user-select: none;
    background: var(--color-background);
    color: var(--color-foreground-level-4);
  }
  td.diff-expand-header {
    user-select: none;
    background: var(--color-background);
    color: var(--color-foreground-level-4);
  }
  td.diff-line-number {
    color: var(--color-foreground-level-4);
  }
  .collapse-button {
    display: flex;
  }
  .collapse-button:hover :global(svg) {
    fill: var(--color-foreground-level-6);
  }
  .created {
    color: var(--color-positive);
    background-color: var(--color-positive-level-1);
  }

  .deleted {
    color: var(--color-negative);
    background-color: var(--color-negative-level-1);
  }
  .diff-type {
    margin-left: 1rem;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
  }
</style>

<article class="changeset-file">
  <header
    on:click={() => {
      collapsed = !collapsed;
    }}>
    <div class="collapse-button">
      {#if collapsed}
        <ChevronRightIcon style="margin-right: 0.5rem; user-select: none;" />
      {:else}
        <ChevronDownIcon style="margin-right: 0.5rem; user-select: none;" />
      {/if}
    </div>
    <FileIcon style="margin-right: 0.5rem;" />
    <p class="typo-text-bold">{file.path}</p>
    {#if label}
      <span
        class="diff-type"
        class:created={label === "created"}
        class:deleted={label === "deleted"}>{label}</span>
    {/if}
  </header>
  {#if !collapsed}
    <main>
      {#if file.diff.type === "plain" && file.diff.hunks.length > 0}
        <table class="diff">
          {#each file.diff.hunks as hunk}
            <tr class="diff-line">
              <td colspan={2} class="diff-expand-action typo-mono" />
              <td colspan={2} class="diff-expand-header typo-mono">
                {hunk.header}
              </td>
            </tr>
            {#each hunk.lines as line}
              <tr class="diff-line" data-expanded data-type={lineSign(line)}>
                <td
                  class="diff-line-number typo-mono left"
                  data-type={lineSign(line)}>
                  {lineNumberL(line)}
                </td>
                <td
                  class="diff-line-number typo-mono right"
                  data-type={lineSign(line)}>
                  {lineNumberR(line)}
                </td>
                <td class="diff-line-type typo-mono" data-type={line.type}>
                  {lineSign(line)}
                </td>
                <td class="diff-line-content typo-mono">{line.line}</td>
              </tr>
            {/each}
          {/each}
        </table>
      {:else}
        <div class="binary">Binary file</div>
      {/if}
    </main>
  {/if}
</article>
