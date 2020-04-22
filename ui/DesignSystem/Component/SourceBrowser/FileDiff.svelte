<script>
  import { Icon } from "../../Primitive";

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
  .changeset-file-path {
    font-weight: 600;
    margin-left: 0.5rem;
  }
  .changeset-file main {
    overflow-x: auto;
  }

  table.diff {
    table-layout: fixed;
    border-collapse: collapse;
  }

  tr.diff-line[data-type="+"] > * {
    background: var(--color-positive-level-1);
  }
  tr.diff-line[data-type="-"] > * {
    background: var(--color-negative-level-1);
  }
  td.diff-line-number {
    text-align: center;
    user-select: none;
    line-height: 150%;
    padding: 0 0.5rem;
  }
  td.diff-line-content {
    white-space: pre;
    width: 100%;
  }
  td.diff-line-type {
    color: var(--color-foreground-level-6);
    user-select: none;
    padding: 0 0.5rem;
    text-align: center;
  }
  td.diff-expand-action {
    text-align: center;
    user-select: none;
  }
  td.diff-expand-header {
    padding-left: 0.5rem;
    user-select: none;
  }

  td.diff-expand-header,
  td.diff-expand-action,
  td.diff-line-number {
    color: var(--color-foreground-level-5);
    background-color: var(--color-foreground-level-1);
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
    <Icon.File />
    <span class="changeset-file-path">{file.path}</span>
  </header>
  <main>
    <table class="diff">
      {#each file.hunks as hunk}
        {#if hunk.expanded}
          {#each hunk.lines as line}
            <tr class="diff-line" data-expanded data-type={line.type}>
              <td class="diff-line-number">{line.num[0] || ''}</td>
              <td class="diff-line-number">{line.num[1] || ''}</td>
              <td class="diff-line-type">{line.type}</td>
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
