<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project } from "ui/src/project";
  import type { GroupedCommitsHistory } from "ui/src/source";

  import { isMaintainer } from "ui/src/project";
  import * as Patch from "ui/src/project/patch";
  import * as Session from "ui/src/session";
  import * as router from "ui/src/router";

  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Button from "design-system/Button.svelte";
  import Markdown from "design-system/Markdown.svelte";
  import MergeIcon from "design-system/icons/Merge.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  import BackButton from "../BackButton.svelte";
  import CompareBranches from "./CompareBranches.svelte";
  import History from "./SourceBrowser/History.svelte";

  export let project: Project;
  export let patch: Patch.Patch;
  export let commits: GroupedCommitsHistory;

  const session = Session.unsealed();

  $: iconColor = patch.merged
    ? "var(--color-negative);"
    : "var(--color-positive);";
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
    color: var(--color-foreground-level-5);
    display: flex;
    align-items: center;
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
        <RevisionIcon style={`fill: ${iconColor};`} />
        {#if patch}
          <h2>
            {#if patch.title}{patch.title}{:else}{patch.id}{/if}
          </h2>
        {/if}
      </div>
    </div>
    <div class="metadata">
      <span style="margin-right: 0.5rem;">Opened by</span>
      {#if patch.identity}
        <UserIdentity
          modalStyle="top: 0.5rem; left: 3rem;"
          urn={patch.identity.urn}
          handle={patch.identity.metadata.handle} />
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
      patchUrl={router.routeToUri({
        type: "project",
        params: {
          urn: project.urn,
          activeView: {
            type: "patch",
            peerId: patch.peerId,
            id: patch.id,
          },
        },
      })}
      compareBranch={{ id: patch.id, peerId: patch.peerId }} />
    <div class="buttons">
      <CommandModal
        dataCy="checkout-patch-modal-toggle"
        let:prop={toggleDropdown}
        command={[
          `upstream patch fetch ${Patch.handle(patch)}`,
          `git checkout ${Patch.TAG_PREFIX}${Patch.handle(patch)}`,
        ].join("\n")}
        description="To fetch and check out this patch in your working copy, run the following commands:">
        <Button
          variant="transparent"
          icon={ArrowBoxUpRightIcon}
          on:click={toggleDropdown}>Checkout patch</Button>
      </CommandModal>
      {#if isMaintainer(session.identity.urn, project) && !patch.merged}
        <CommandModal
          dataCy="merge-patch-modal-toggle"
          let:prop={toggleDropdown}
          command={[
            `upstream patch fetch ${Patch.handle(patch)}`,
            `git merge ${Patch.TAG_PREFIX}${Patch.handle(patch)}`,
            `rad push`,
          ].join("\n")}
          description="To merge this patch and publish the changes, run these commands in your working copy:">
          <Button icon={MergeIcon} on:click={toggleDropdown}>Merge</Button>
        </CommandModal>
      {/if}
    </div>
  </div>
  <History projectUrn={project.urn} history={commits} />
</div>
