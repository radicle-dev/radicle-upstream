<script lang="typescript">
  import type { Project } from "../../src/project";
  import { isMaintainer } from "../../src/project";

  import List from "./List.svelte";
  import ProjectCard from "./ProjectCard.svelte";
  import Stats from "./Stats.svelte";

  export let projects: Project[];
  export let userUrn: string;
  export let orgAddress: string;

  const projectCardProps = (project: Project) => ({
    title: project.metadata.name,
    description: project.metadata.description || "",
    showMaintainerBadge: isMaintainer(userUrn, project),
    anchor: project.anchor,
    orgAddress,
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
      <Stats
        branches={project.stats.branches}
        commits={project.stats.commits}
        contributors={project.stats.contributors} />
    {/if}
  </div>
</List>
