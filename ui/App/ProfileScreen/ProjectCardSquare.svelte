<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as project from "ui/src/project";

  import { Avatar, Badge, CopyableIdentifier } from "ui/DesignSystem";
  import ProjectAnchorPopover from "ui/App/OrgScreen/ProjectAnchorPopover.svelte";
  import ProjectStats from "./ProjectStats.svelte";

  interface Stats {
    branches: number;
    commits: number;
    contributors: number;
  }

  export let title: string;
  export let urn: string;
  export let description: string | undefined = undefined;
  export let maintainerUrn: string;
  export let showMaintainerBadge: boolean = false;
  export let anchor: project.Anchor | undefined;
  export let stats: Stats | undefined;
</script>

<style>
  .project-card {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    padding: 2rem;
    height: 15rem;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
    cursor: pointer;
  }
  .title-row {
    display: flex;
    margin-bottom: 1rem;
    align-items: center;
  }

  .desc {
    margin-top: 0.75rem;
    color: var(--color-foreground-level-6);
    max-height: 3rem;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .bottom {
    display: flex;
    justify-content: space-between;
  }
</style>

<li class="project-card" on:click>
  <div>
    <div class="title-row">
      <h2 class="typo-overflow-ellipsis">{title}</h2>
      {#if showMaintainerBadge}
        <Badge style="margin-left: 0.5rem" variant="maintainer" />
      {/if}
    </div>
    <CopyableIdentifier kind="radicleId" value={urn} />
    {#if description}
      <p class="desc">{description}</p>
    {/if}
    {#if anchor}
      <ProjectAnchorPopover {anchor} replicated={true} />
    {/if}
  </div>
  <div class="bottom">
    {#if stats}
      <ProjectStats
        branches={stats.branches}
        commits={stats.commits}
        contributors={stats.contributors} />
    {/if}
    <Avatar
      size="small"
      kind={{
        type: "userEmoji",
        uniqueIdentifier: maintainerUrn,
      }} />
  </div>
</li>
