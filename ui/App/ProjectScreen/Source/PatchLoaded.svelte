<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { GroupedCommitsHistory } from "ui/src/source";
  import type { PatchView } from "../route";

  import * as Patch from "ui/src/project/patch";
  import * as router from "ui/src/router";
  import { Project } from "ui/src/project";
  import { unreachable } from "ui/src/unreachable";

  import ChatIcon from "design-system/icons/Chat.svelte";
  import CommitIcon from "design-system/icons/Commit.svelte";
  import LinkIcon from "design-system/icons/Link.svelte";
  import Markdown from "design-system/Markdown.svelte";

  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  import BackButton from "../BackButton.svelte";
  import History from "./SourceBrowser/History.svelte";
  import PatchIcon from "./PatchIcon.svelte";
  import PatchDiscussion from "./PatchDiscussion.svelte";

  export let project: Project;
  export let patch: Patch.Patch;
  export let commits: GroupedCommitsHistory;
  export let view: PatchView;

  function tabs(view: PatchView, project: Project, patch: Patch.Patch) {
    return [
      {
        title: "Commits",
        active: view === "commits",
        icon: CommitIcon,
        counter: commits.stats.commits,
        onClick: () => {
          router.push({
            type: "project",
            params: {
              urn: project.urn,
              activeView: {
                type: "patch",
                peerId: patch.peerId,
                id: patch.id,
                view: "commits",
              },
            },
          });
        },
      },
      {
        title: "Discussion",
        active: view === "discussion",
        icon: ChatIcon,
        counter: patch.comments.length,
        onClick: () => {
          router.push({
            type: "project",
            params: {
              urn: project.urn,
              activeView: {
                type: "patch",
                peerId: patch.peerId,
                id: patch.id,
                view: "discussion",
              },
            },
          });
        },
      },
    ];
  }
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
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .divider {
    border-bottom: 1px solid var(--color-foreground-level-3);
    height: 1px;
    margin: 0.75rem 0 2rem 0;
  }

  .copyable-link {
    cursor: pointer;
    display: flex;
  }
  .copyable-link :global(svg):hover {
    fill: var(--color-foreground-level-4);
  }
</style>

<div class="patch-page" data-cy="patch-page">
  <BackButton
    style="padding: 1rem 1rem 1.5rem 1rem;"
    on:arrowClick={() => {
      router.push({
        type: "project",
        params: {
          urn: project.urn,
          activeView: { type: "patches", filter: "open" },
        },
      });
    }}>
    <div>
      <div class="title" data-cy="patch-title">
        <PatchIcon status={patch.status.current} />
        {#if patch}
          <h2>
            {#if patch.title}{patch.title}{:else}{patch.id}{/if}
          </h2>
          <div
            role="button"
            title="Copy patch URL to clipboard"
            class="copyable-link button-transition">
            <LinkIcon
              on:click={() => {
                Patch.copyPatchUrlToClipboard(project.urn, patch);
              }} />
          </div>
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
  <TabBar style="margin-top: 1.5rem" tabs={tabs(view, project, patch)} />
  <div class="divider" />
  {#if view === "commits"}
    <History projectUrn={project.urn} history={commits} />
  {:else if view === "discussion"}
    <PatchDiscussion projectUrn={project.urn} {patch} />
  {:else}
    {unreachable(view)}
  {/if}
</div>
