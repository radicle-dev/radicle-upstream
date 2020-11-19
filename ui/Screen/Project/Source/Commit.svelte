<script lang="typescript">
  import { pop } from "svelte-spa-router";

  import { formatCommitTime } from "../../../src/source";
  import {
    commit as store,
    fetchCommit,
  } from "../../../src/screen/project/source";
  import type { Urn } from "../../../src/urn";

  import { Icon } from "../../../DesignSystem/Primitive";
  import { Header, Remote } from "../../../DesignSystem/Component";

  import Changeset from "../../../DesignSystem/Component/SourceBrowser/Changeset.svelte";

  export let params: { hash: string; urn: Urn };
  const { hash } = params;

  fetchCommit(hash);
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
    border-radius: 4px;
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
  <Remote {store} let:data={commit}>
    <Header.Back style="padding: 1rem; z-index: 0;" on:arrowClick={() => pop()}>
      <h3 style="margin-bottom: .75rem">{commit.header.summary}</h3>
      <div class="metadata">
        <span class="field">
          <!-- NOTE(cloudhead): These awful margin hacks are here because
            there is a bug in prettier that breaks our HTML if we try to format
            it differently. -->
          <span>{commit.header.author.name}</span>
          <span>committed</span>
          <span class="typo-mono">{commit.header.sha1.substring(0, 7)}</span>
          <span style="margin-right: -1ch">to</span>
          <span class="branch typo-semi-bold">
            <Icon.Branch
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-6)" />
            <span style="margin-left: -0.5ch">{commit.branch}</span>
          </span>
          <span style="margin-left: -0.5ch">
            {formatCommitTime(commit.header.committerTime)}
          </span>
        </span>
      </div>
    </Header.Back>
    <div class="content" data-cy="commit-header">
      <pre
        class="typo-mono"
        style="margin-bottom: 1rem">
        {commit.header.summary}
      </pre>
      <pre
        class="description"
        style="margin-bottom: 1rem">
        {commit.header.description}
      </pre>
      <hr />
      <div class="context">
        <div>
          <p class="field">
            Authored by
            <span class="author typo-semi-bold">
              {commit.header.author.name}
            </span>
            <span class="typo-mono">&lt;{commit.header.author.email}&gt;</span>
          </p>
          {#if commit.header.committer.email != commit.header.author.email}
            <p class="field">
              Committed by
              <span class="author typo-semi-bold">
                {commit.header.committer.name}
              </span>
              <span class="typo-mono">
                &lt;{commit.header.committer.email}&gt;
              </span>
            </p>
          {/if}
        </div>
        <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
        <p class="field">
          Commit
          <span class="hash">{commit.header.sha1}</span>
        </p>
      </div>
    </div>

    <main>
      <Changeset diff={commit.diff} stats={commit.stats} />
    </main>
  </Remote>
</div>
