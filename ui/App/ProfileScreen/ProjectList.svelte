<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project } from "ui/src/project";
  import { isMaintainer } from "ui/src/project";

  import { List } from "ui/DesignSystem";
  import ProjectCard from "./ProjectCard.svelte";
  import ProjectStats from "ui/App/SharedComponents/ProjectStats.svelte";

  export let projects: Project[];
  export let userUrn: string;

  const projectCardProps = (project: Project) => ({
    title: project.metadata.name,
    description: project.metadata.description || "",
    showMaintainerBadge: isMaintainer(userUrn, project),
    anchor: project.anchor,
  });
</script>

<style>
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }
</style>

<List
  dataCy="project-list"
  items={projects}
  on:select
  let:item={project}
  style="margin: 0 auto;">
  <div
    class="list-item"
    data-cy={`project-list-entry-${project.metadata.name}`}>
    <ProjectCard {...projectCardProps(project)} />

    {#if project.stats}
      <ProjectStats
        branches={project.stats.branches}
        commits={project.stats.commits}
        contributors={project.stats.contributors} />
    {/if}
  </div>
</List>
