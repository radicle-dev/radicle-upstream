<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";

  import { formatCommitTime, Commit } from "ui/src/source";
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";
  import { unreachable } from "ui/src/unreachable";
  import * as mutexExecutor from "ui/src/mutexExecutor";

  import BranchIcon from "design-system/icons/Branch.svelte";
  import Loading from "design-system/Loading.svelte";

  import AnchorCard from "./AnchorCard.svelte";
  import BackButton from "../BackButton.svelte";
  import Changeset from "./SourceBrowser/Changeset.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";

  export let projectUrn: string;
  export let commitHash: string;
  export let anchors: project.ConfirmedAnchor[];

  let commitResult:
    | { type: "loading" }
    | { type: "error" }
    | { type: "ok"; commit: Commit } = { type: "loading" };

  const commitLoader = mutexExecutor.create();

  $: {
    loadCommit(commitHash, projectUrn);
  }

  async function loadCommit(
    commitId: string,
    projectUrn: string
  ): Promise<void> {
    commitResult = { type: "loading" };
    try {
      const commit = await commitLoader.run(abort =>
        proxy.client.source.commitGet({ projectUrn, sha1: commitId }, { abort })
      );
      if (commit) {
        commitResult = { type: "ok", commit };
      }
    } catch (err: unknown) {
      commitResult = { type: "error" };
      const e = error.fromUnknown(err);
      if (!e.message.match("object not found")) {
        notification.showException(
          new error.Error({
            code: error.Code.CommitFetchFailure,
            message: "Could not fetch commit",
            source: err,
          })
        );
      }
    }
  }
</script>

<style>
  .commit-page {
    max-width: var(--content-max-width);
    margin: 0 auto;
    padding: 0 var(--content-padding);
    min-width: var(--content-min-width);
  }

  .commit-header {
    background: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
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

  .loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: calc(100vh - var(--bigheader-height) - var(--topbar-height));
  }
</style>

<div class="commit-page" data-cy="commit-page">
  {#if commitResult.type === "loading"}
    <div class="loading-container">
      <Loading />
    </div>
  {:else if commitResult.type === "ok"}
    <BackButton
      style="padding: 1rem; z-index: 0;"
      on:arrowClick={() => router.pop()}>
      <h3 style="margin-bottom: .75rem">
        {commitResult.commit.header.summary}
      </h3>
      <div>
        <span class="field">
          <!-- NOTE(cloudhead): These awful margin hacks are here because
            there is a bug in prettier that breaks our HTML if we try to format
            it differently. -->
          <span>{commitResult.commit.header.author.name}</span>
          <span>committed</span>
          <CopyableIdentifier
            style="display: inline-block;"
            kind="commitHash"
            value={commitResult.commit.header.sha1} />
          {#if commitResult.commit.branches.length > 0}
            <span style="margin-right: -1ch">to</span>
            <span class="branch typo-semi-bold">
              <BranchIcon
                style="vertical-align: bottom; fill:
                var(--color-foreground-level-6)" />
              <span data-cy="commit-branch" style="margin-left: -0.5ch"
                >{commitResult.commit.branches[0]}</span>
            </span>
          {/if}
          <span style="margin-left: -0.5ch">
            {formatCommitTime(commitResult.commit.header.committerTime)}
          </span>
        </span>
      </div>
    </BackButton>
    <div class="commit-header" data-cy="commit-header">
      <pre class="typo-mono" style="margin-bottom: 1rem">
        {commitResult.commit.header.summary}
      </pre>
      <pre class="description" style="margin-bottom: 1rem">
        {commitResult.commit.header.description}
      </pre>
      <hr />
      <div class="context">
        <div>
          <span class="field">
            Authored by
            <span class="author typo-semi-bold">
              {commitResult.commit.header.author.name}
            </span>
            <span class="typo-mono"
              >&lt;{commitResult.commit.header.author.email}&gt;</span>
          </span>
          {#if commitResult.commit.header.committer.email !== commitResult.commit.header.author.email}
            <span class="field">
              Committed by
              <span class="author typo-semi-bold">
                {commitResult.commit.header.committer.name}
              </span>
              <span class="typo-mono">
                &lt;{commitResult.commit.header.committer.email}&gt;
              </span>
            </span>
          {/if}
        </div>
        <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
        <span class="field">
          Commit
          <CopyableIdentifier
            tooltipPosition="left"
            style="display: inline-block;"
            kind="commitHash"
            value={commitResult.commit.header.sha1} />
        </span>
      </div>
    </div>

    {#each anchors as anchor}
      <AnchorCard {anchor} showCommitHash={false} style="margin-top: 1rem;" />
    {/each}

    <main>
      <Changeset
        diff={commitResult.commit.diff}
        stats={commitResult.commit.stats} />
    </main>
  {:else if commitResult.type === "error"}
    <EmptyState emoji="🦤">
      Commit <CopyableIdentifier
        kind="commitHash"
        value={commitHash}
        style="display: inline-block;" /> isn’t replicated yet.
    </EmptyState>
  {:else}
    {unreachable(commitResult)}
  {/if}
</div>
