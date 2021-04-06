<script lang="typescript">
  import { pop } from "svelte-spa-router";

  import { isMaintainer } from "../../../src/project";
  import type { Project } from "../../../src/project";
  import type { UnsealedSession } from "../../../src/session";
  import { selectCommit } from "../../../src/screen/project/source";
  import type { GrouppedCommitsHistory } from "../../../src/source";
  import type { MergeRequest } from "../../../src/project/mergeRequest";

  import { Avatar, Icon, Markdown } from "../../../DesignSystem/Primitive";
  import { CompareBranches, Header } from "../../../DesignSystem/Component";
  import History from "../../../DesignSystem/Component/SourceBrowser/History.svelte";
  import CheckoutMergeRequestButton from "./CheckoutMergeRequestButton.svelte";
  import AcceptMergeRequestButton from "./AcceptMergeRequestButton.svelte";

  export let project: Project;
  export let mergeRequest: MergeRequest;
  export let commits: GrouppedCommitsHistory;
  export let session: UnsealedSession;

  const mergeInfo = mergeRequest && mergeRequest.merged ? "Closed" : "Opened";
  const iconColor =
    mergeRequest && mergeRequest.merged
      ? "var(--color-negative);"
      : "var(--color-positive);";
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
    align-items: center;
    gap: 0.5rem;
    margin: -0.1875rem 0 0.5rem;
  }

  .metadata {
    display: flex;
    align-items: center;
    color: var(--color-foreground-level-5);
  }

  .desc {
    border-top: 1px solid var(--color-foreground-level-3);
    padding: 1.5rem;
  }

  .action-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    background: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .action-box .buttons {
    display: flex;
    gap: 1rem;
  }
</style>

<div class="merge-request-page" data-cy="merge-request-page">
  <Header.Back style="padding: 1rem; z-index: 0;" on:arrowClick={() => pop()}>
    <div>
      <div class="title">
        <Icon.Revision style={`fill: ${iconColor};`} />
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
    <div class="desc">
      <Markdown content={mergeRequest.description} />
    </div>
  {/if}
  <div class="action-box">
    <CompareBranches
      baseBranch={project.metadata.defaultBranch}
      compareBranch={mergeRequest.id} />
    <div class="buttons">
      <CheckoutMergeRequestButton
        id={mergeRequest.id}
        peerId={mergeRequest.peerId} />
      {#if isMaintainer(session.identity.urn, project)}
        <AcceptMergeRequestButton id={mergeRequest.id} />
      {/if}
    </div>
  </div>
  <History history={commits} on:select={event => selectCommit(event.detail)} />
</div>
