<script>
  import { format } from "timeago.js";

  import { showNotification } from "../../store/notification.js";
  import { commit, fetchCommit } from "../../src/source.ts";
  import * as remote from "../../src/remote.ts";

  import { Title, Flex, Icon } from "../../DesignSystem/Primitive";

  import FileDiff from "../../DesignSystem/Component/SourceBrowser/FileDiff.svelte";

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
      <FileDiff {file} />
    {/each}
  </main>
{/if}
