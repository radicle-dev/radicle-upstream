<script lang="typescript">
  import * as error from "ui/src/error";
  import { formatCommitTime } from "ui/src/source";
  import { commit, fetchCommit } from "ui/src/screen/project/source";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";

  import { Icon } from "ui/DesignSystem/Primitive";
  import BackButton from "ui/Screen/Project/BackButton.svelte";

  import Changeset from "ui/DesignSystem/Component/SourceBrowser/Changeset.svelte";

  export let commitHash: string;

  $: {
    if ($commit.status === remote.Status.Error) {
      error.show($commit.error);
    }
  }

  fetchCommit(commitHash);
</script>

<style>
  .commit-page {
    max-width: var(--content-max-width);
    margin: 0 auto;
    padding: 0 var(--content-padding);
    min-width: var(--content-min-width);
  }

  .content {
    background: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 1.5rem;
  }

  .field {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }

  .field:last-child {
    margin-bottom: 0;
  }

  .branch {
    margin: 0 0.5rem;
    color: var(--color-foreground-level-6);
  }

  .context {
    align-items: flex-end;
    display: flex;
    justify-content: space-between;
  }

  .author {
    color: var(--color-foreground);
  }

  /* TODO(cloudhead): These should be global */
  hr {
    border: 0;
    border-top: 1px solid var(--color-foreground-level-3);
    margin: 1rem 0 1.5rem 0;
  }
</style>

<div class="commit-page" data-cy="commit-page">
  {#if $commit.status === remote.Status.Success}
    <BackButton
      style="padding: 1rem; z-index: 0;"
      on:arrowClick={() => router.pop()}>
      <h3 style="margin-bottom: .75rem">{$commit.data.header.summary}</h3>
      <div class="metadata">
        <span class="field">
          <!-- NOTE(cloudhead): These awful margin hacks are here because
            there is a bug in prettier that breaks our HTML if we try to format
            it differently. -->
          <span>{$commit.data.header.author.name}</span>
          <span>committed</span>
          <span class="typo-mono"
            >{$commit.data.header.sha1.substring(0, 7)}</span>
          {#if $commit.data.branches.length > 0}
            <span style="margin-right: -1ch">to</span>
            <span class="branch typo-semi-bold">
              <Icon.Branch
                style="vertical-align: bottom; fill:
                var(--color-foreground-level-6)" />
              <span data-cy="commit-branch" style="margin-left: -0.5ch"
                >{$commit.data.branches[0]}</span>
            </span>
          {/if}
          <span style="margin-left: -0.5ch">
            {formatCommitTime($commit.data.header.committerTime)}
          </span>
        </span>
      </div>
    </BackButton>
    <div class="content" data-cy="commit-header">
      <pre
        class="typo-mono"
        style="margin-bottom: 1rem">
        {$commit.data.header.summary}
      </pre>
      <pre
        class="description"
        style="margin-bottom: 1rem">
        {$commit.data.header.description}
      </pre>
      <hr />
      <div class="context">
        <div>
          <p class="field">
            Authored by
            <span class="author typo-semi-bold">
              {$commit.data.header.author.name}
            </span>
            <span class="typo-mono"
              >&lt;{$commit.data.header.author.email}&gt;</span>
          </p>
          {#if $commit.data.header.committer.email != $commit.data.header.author.email}
            <p class="field">
              Committed by
              <span class="author typo-semi-bold">
                {$commit.data.header.committer.name}
              </span>
              <span class="typo-mono">
                &lt;{$commit.data.header.committer.email}&gt;
              </span>
            </p>
          {/if}
        </div>
        <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
        <p class="field">
          Commit
          <span class="hash">{$commit.data.header.sha1}</span>
        </p>
      </div>
    </div>

    <main>
      <Changeset diff={$commit.data.diff} stats={$commit.data.stats} />
    </main>
  {/if}
</div>
