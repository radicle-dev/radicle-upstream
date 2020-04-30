<script>
  import { Title, Icon } from "../../Primitive";

  export let file = null;
</script>

<style>
  /* TODO(cloudhead): Reconcile with `FileSource`? */
  .changeset-file {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
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
    background: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }
  td.diff-expand-header {
    user-select: none;
    background: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }

  td.diff-line-number {
    color: var(--color-foreground-level-4);
  }

  td.diff-expand-header,
  td.diff-expand-action,
  td.diff-line-type,
  td.diff-line-content,
  td.diff-line-number {
    font-family: var(--typeface-mono-regular);
  }
</style>

<article class="changeset-file">
  <header>
    <Icon.File style="margin-right: 8px;" />
    <Title>{file.path}</Title>
  </header>
  <main>
    <table class="diff">
      {#each file.hunks as hunk}
        {#if hunk.expanded}
          {#each hunk.lines as line}
            <tr class="diff-line" data-expanded data-type={line.type}>
              <td class="diff-line-number left" data-type={line.type}>
                {line.num[0] || ''}
              </td>
              <td class="diff-line-number right" data-type={line.type}>
                {line.num[1] || ''}
              </td>
              <td class="diff-line-type" data-type={line.type}>{line.type}</td>
              <td class="diff-line-content">{line.content}</td>
            </tr>
          {/each}
        {:else}
          <tr class="diff-line">
            <td colspan="2" class="diff-expand-action">...</td>
            <td colspan="2" class="diff-expand-header">{hunk.header}</td>
          </tr>
        {/if}
      {/each}
    </table>
  </main>
</article>
