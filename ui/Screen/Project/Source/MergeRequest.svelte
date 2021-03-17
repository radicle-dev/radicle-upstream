<script lang="typescript">
  import { pop, querystring } from "svelte-spa-router";
  import { getContext } from "svelte";

  import { isMaintainer } from "../../../src/project";
  import { parseQueryString } from "../../../src/path";
  import type { UnsealedSession } from "../../../src/session";
  import {
    mergeRequestCommits as store,
    fetchMergeRequestCommits,
    selectCommit,
  } from "../../../src/screen/project/source";
  import type { CommitHeader } from "../../../src/source";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import { Header, Remote } from "../../../DesignSystem/Component";
  import History from "../../../DesignSystem/Component/SourceBrowser/History.svelte";
  import CheckoutMergeRequestButton from "./CheckoutMergeRequestButton.svelte";
  import AcceptMergeRequestButton from "./AcceptMergeRequestButton.svelte";

  const onSelect = ({ detail: commit }: { detail: CommitHeader }) => {
    selectCommit(commit);
  };

  const session = getContext("session") as UnsealedSession;

  const parsed = parseQueryString($querystring);
  const mergeRequest = parsed.mergeRequest;
  const mergeInfo =
    mergeRequest.mergeRequest && mergeRequest.mergeRequest.merged
      ? "Closed"
      : "Opened";
  fetchMergeRequestCommits(mergeRequest);
</script>

<style>
  .merge-request-page {
    max-width: var(--content-max-width);
    margin: 0 auto;
    padding: 0 var(--content-padding);
    min-width: var(--content-min-width);
  }

  .title {
    display: flex;
  }

  .metadata {
    display: flex;
    flex-direction: column;
  }

  .row {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }

  .row:last-child {
    margin-bottom: 0;
  }

  .action-box {
    width: 100%;
  }
</style>

<div class="merge-request-page" data-cy="merge-request-page">
  <Header.Back style="padding: 1rem; z-index: 0;" on:arrowClick={() => pop()}>
    <div>
      <div class="title">
        <Icon.Revision style="fill: var(--color-positive);" />
        {#if mergeRequest.title}
          <h2>{mergeRequest.title}</h2>
        {/if}
      </div>
    </div>
    <div class="metadata">
      <span> {mergeInfo} by </span>
      {#if mergeRequest.identity}
        <Avatar
          avatarFallback={mergeRequest.identity.avatarFallback}
          size="small"
          style="display: flex; justify-content: flex-start; margin-left: 0.5rem;"
          title={mergeRequest.identity.metadata.handle}
          variant="circle" />
      {:else}
        <p style="margin-left: 0.5rem;">{mergeRequest.peerId}</p>
      {/if}
    </div>
  </Header.Back>
  {#if mergeRequest.description}
    <pre class="row">
      {mergeRequest.description}
    </pre>
  {/if}
  <div class="action-box">
    <!-- put the branch stuff here -->
    <div class="buttons">
      <CheckoutMergeRequestButton
        id={mergeRequest.id}
        peerId={mergeRequest.identity.peerId} />
      {#if isMaintainer(session.identity.urn, getContext('project-page').project)}
        <AcceptMergeRequestButton id={mergeRequest.id} />
      {/if}
    </div>
  </div>
  <Remote {store} let:data={history}>
    <History {history} on:select={onSelect} />
  </Remote>
</div>
