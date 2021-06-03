<script lang="typescript">
  import {
    lineNumberL,
    lineNumberR,
    lineSign,
    FileDiffType,
  } from "../../../src/source/diff";
  import type { ModifiedFile } from "../../../src/source/diff";

  import { Icon } from "../../Primitive";

  export let file: ModifiedFile;
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
    height: 3rem;
    display: flex;
    align-items: center;
    background: none;
    border-bottom: 1px solid var(--color-foreground-level-3);
    border-radius: 0;
    padding: 0.75rem;
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
</style>

<article class="changeset-file">
  <header>
    <Icon.File style="margin-right: 8px;" />
    <p class="typo-text-bold">{file.path}</p>
  </header>
  <main>
    {#if file.diff.type === FileDiffType.Plain && file.diff.hunks.length > 0}
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
</article>
