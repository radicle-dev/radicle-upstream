<script>
  import { format } from "timeago.js";

  import { showNotification } from "../../store/notification.js";
  import { commit, fetchCommit } from "../../src/source.ts";
  import * as remote from "../../src/remote.ts";

  import { Title, Flex, Icon } from "../../DesignSystem/Primitive";

  export let params = null;
  const projectId = params.id;
  const commitHash = params.hash;

  $: if ($commit.status === remote.Status.Error) {
    console.log($commit.error);
    showNotification({
      text: "Could not fetch commit",
      level: "error"
    });
  }

  fetchCommit({ projectId, sha1: commitHash });
</script>

<style>
  header {
    background: var(--color-foreground-level-1);
    border-radius: 4px;
    padding: 1.5rem;
  }
  .description {
    font-family: var(--typeface-mono-regular);
  }
  .field {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }
  .field:last-child {
    margin-bottom: 0;
  }
  .email {
    font-family: var(--typeface-mono-regular);
  }
  .branch {
    margin: 0 0.5rem;
    font-weight: bold;
    color: var(--color-foreground-level-6);
  }
  .author {
    font-weight: bold;
    color: var(--color-foreground);
  }
  .hash {
    font-family: var(--typeface-mono-regular);
  }

  .changeset-summary {
    margin-top: 2rem;
    margin-bottom: 1.5rem;
    margin-left: 1.5rem;
  }
  .changeset-summary .additions {
    color: var(--color-positive);
    font-weight: 600;
  }
  .changeset-summary .deletions {
    color: var(--color-negative);
    font-weight: 600;
  }

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

  /* TODO(cloudhead): These should be global */
  a {
    color: var(--color-secondary);
  }
  hr {
    border: 0;
    border-top: 1px solid var(--color-foreground-level-3);
    margin: 1rem 0 1.5rem 0;
  }
</style>

{#if $commit.status === remote.Status.Success}
  <header>
    <Flex style="align-items: flex-start">
      <div slot="left">
        <Title variant="large" style="margin-bottom: 1rem">
          {$commit.data.summary}
        </Title>
      </div>
      <div slot="right">
        <span class="field">
          <!-- NOTE(cloudhead): These awful margin hacks are here because
          there is a bug in prettier that breaks our HTML if we try to format
          it differently. -->
          <span style="margin-right: -1ch">Committed to</span>
          <span class="branch">
            <Icon.Branch
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-6)" />
            <span style="margin-left: -0.5ch">{$commit.data.branch}</span>
          </span>
          <span style="margin-left: -0.5ch">
            {format($commit.data.committerTime * 1000)}
          </span>
        </span>
      </div>
    </Flex>
    <pre class="description" style="margin-bottom: 1rem">
      {$commit.data.description}
    </pre>
    <hr />
    <Flex style="align-items: flex-end">
      <div slot="left">
        <p class="field">
          Authored by
          <span class="author">{$commit.data.author.name}</span>
          <span class="email">&lt;{$commit.data.author.email}&gt;</span>
        </p>
        {#if $commit.data.committer.email != $commit.data.author.email}
          <p class="field">
            Committed by
            <span class="author">{$commit.data.committer.name}</span>
            <span class="email">&lt;{$commit.data.committer.email}&gt;</span>
          </p>
        {/if}
      </div>
      <div slot="right">
        <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
        <p class="field">
          Commit
          <span class="hash">{$commit.data.sha1}</span>
        </p>
      </div>
    </Flex>
  </header>
  <main>
    <div class="changeset-summary">
      {$commit.data.changeset.files.length} file(s) changed with
      <span class="additions">
        {$commit.data.changeset.summary.additions} addition(s)
      </span>
      and
      <span class="deletions">
        {$commit.data.changeset.summary.deletions} deletion(s)
      </span>
    </div>
    {#each $commit.data.changeset.files as file}
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
    {/each}
  </main>
{/if}
