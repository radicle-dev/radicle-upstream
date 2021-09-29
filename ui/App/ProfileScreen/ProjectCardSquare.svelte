<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";

  import { Avatar, Badge, CopyableIdentifier } from "ui/DesignSystem";
  import ProjectAnchorPopover from "ui/App/OrgScreen/ProjectAnchorPopover.svelte";
  import ProjectStats from "./ProjectStats.svelte";

  export let project: Project;
  export let isMaintainer: boolean;
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

  .project-card:hover {
    background-color: var(--color-foreground-level-1);
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

<div
  class="project-card"
  data-cy={`project-list-entry-${project.metadata.name}`}
  on:click>
  <div>
    <div class="title-row">
      <h2 class="typo-overflow-ellipsis">{project.metadata.name}</h2>
      {#if isMaintainer}
        <Badge style="margin-left: 0.5rem" variant="maintainer" />
      {/if}
    </div>
    <CopyableIdentifier kind="radicleId" value={project.urn} />
    {#if project.metadata.description}
      <p class="desc">{project.metadata.description}</p>
    {/if}
    {#if project.anchor}
      <ProjectAnchorPopover anchor={project.anchor} replicated={true} />
    {/if}
  </div>
  <div class="bottom">
    <ProjectStats
      branches={project.stats.branches}
      commits={project.stats.commits}
      contributors={project.stats.contributors} />
    <Avatar
      size="small"
      kind={{
        type: "userEmoji",
        uniqueIdentifier: project.metadata.maintainers[0],
      }} />
  </div>
</div>
