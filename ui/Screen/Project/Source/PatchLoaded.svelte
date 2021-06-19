<script lang="typescript">
  import * as router from "ui/src/router";
  import { isMaintainer } from "ui/src/project";
  import type { Project } from "ui/src/project";
  import type { UnsealedSession } from "ui/src/session";
  import { selectCommit } from "ui/src/screen/project/source";
  import type { GroupedCommitsHistory } from "ui/src/source";
  import type { Patch } from "ui/src/project/patch";

  import { Avatar, Icon, Markdown } from "ui/DesignSystem";
  import { CompareBranches } from "ui/DesignSystem";
  import History from "ui/DesignSystem/SourceBrowser/History.svelte";
  import CheckoutPatchButton from "./CheckoutPatchButton.svelte";
  import AcceptPatchButton from "./AcceptPatchButton.svelte";
  import BackButton from "../BackButton.svelte";

  export let project: Project;
  export let patch: Patch;
  export let commits: GroupedCommitsHistory;
  export let session: UnsealedSession;

  $: iconColor = patch.merged
    ? "var(--color-negative);"
    : "var(--color-positive);";

  $: peerLabel = patch.identity ? patch.identity.metadata.handle : patch.peerId;
</script>

<style>
  .patch-page {
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

<div class="patch-page" data-cy="patch-page">
  <BackButton
    style="padding: 1rem; z-index: 0;"
    on:arrowClick={() => router.pop()}>
    <div>
      <div class="title" data-cy="patch-title">
        <Icon.Revision style={`fill: ${iconColor};`} />
        {#if patch}
          <h2>
            {#if patch.title}{patch.title}{:else}{patch.id}{/if}
          </h2>
        {/if}
      </div>
    </div>
    <div class="metadata">
      <span> Opened by </span>
      {#if patch.identity}
        <Avatar
          avatarFallback={patch.identity.avatarFallback}
          size="small"
          style="display: flex; justify-content: flex-start; margin-left: 0.5rem;"
          title={patch.identity.metadata.handle}
          variant="circle" />
      {:else}
        <p style="margin-left: 0.5rem;">{patch.peerId}</p>
      {/if}
    </div>
  </BackButton>
  {#if patch.description}
    <div class="desc">
      <Markdown content={patch.description} />
    </div>
  {/if}
  <div class="action-box">
    <CompareBranches
      baseBranch={project.metadata.defaultBranch}
      compareBranch={`${peerLabel}/${patch.id}`} />
    <div class="buttons">
      <CheckoutPatchButton {patch} myPeerId={session.identity.peerId} />
      {#if isMaintainer(session.identity.urn, project) && !patch.merged}
        <AcceptPatchButton {patch} myPeerId={session.identity.peerId} />
      {/if}
    </div>
  </div>
  <History history={commits} on:select={event => selectCommit(event.detail)} />
</div>
