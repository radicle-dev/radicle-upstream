<script>
  import { format } from "timeago.js";

  import * as notification from "../../src/notification.ts";
  import { commit as store, fetchCommit } from "../../src/source.ts";
  import * as remote from "../../src/remote.ts";

  import { Title, Flex, Icon } from "../../DesignSystem/Primitive";
  import { Remote } from "../../DesignSystem/Component";

  import FileDiff from "../../DesignSystem/Component/SourceBrowser/FileDiff.svelte";

  export let params = null;
  const projectId = params.id;
  const commitHash = params.hash;

  $: if ($store.status === remote.Status.Error) {
    console.log($store.error);
    notification.error({ message: "Could not fetch commit" });
  }

  fetchCommit({ projectId, sha1: commitHash });
</script>

<style>
  .commit-page {
    padding-top: 32px;
    margin-bottom: 64px;
    margin-left: 96px;
    margin-right: 96px;
  }

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
    font-family: var(--typeface-medium);
    color: var(--color-foreground-level-6);
  }
  .author {
    font-family: var(--typeface-medium);
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
  .changeset-summary .amount {
    font-family: var(--typeface-medium);
  }
  .changeset-summary .additions {
    color: var(--color-positive);
    font-family: var(--typeface-medium);
  }
  .changeset-summary .deletions {
    color: var(--color-negative);
    font-family: var(--typeface-medium);
  }

  /* TODO(cloudhead): These should be global */
  hr {
    border: 0;
    border-top: 1px solid var(--color-foreground-level-3);
    margin: 1rem 0 1.5rem 0;
  }
</style>

<div class="commit-page">
  <Remote {store} let:data={commit}>
    <header>
      <Flex style="align-items: flex-start">
        <div slot="left">
          <Title variant="large" style="margin-bottom: 1rem">
            {commit.summary}
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
              <span style="margin-left: -0.5ch">{commit.branch}</span>
            </span>
            <span style="margin-left: -0.5ch">
              {format(commit.committerTime * 1000)}
            </span>
          </span>
        </div>
      </Flex>
      <pre class="description" style="margin-bottom: 1rem">
        {commit.description}
      </pre>
      <hr />
      <Flex style="align-items: flex-end">
        <div slot="left">
          <p class="field">
            Authored by
            <span class="author">{commit.author.name}</span>
            <span class="email">&lt;{commit.author.email}&gt;</span>
          </p>
          {#if commit.committer.email != commit.author.email}
            <p class="field">
              Committed by
              <span class="author">{commit.committer.name}</span>
              <span class="email">&lt;{commit.committer.email}&gt;</span>
            </p>
          {/if}
        </div>
        <div slot="right">
          <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
          <p class="field">
            Commit
            <span class="hash">{commit.sha1}</span>
          </p>
        </div>
      </Flex>
    </header>
    <main>
      <div class="changeset-summary">
        <span class="amount">
          {commit.changeset.files.length} file(s) changed
        </span>
        with
        <span class="additions">
          {commit.changeset.summary.additions} addition(s)
        </span>
        and
        <span class="deletions">
          {commit.changeset.summary.deletions} deletion(s)
        </span>
      </div>
      {#each commit.changeset.files as file}
        <FileDiff {file} />
      {/each}
    </main>
  </Remote>
</div>
