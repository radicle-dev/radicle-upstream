<script lang="ts">
  import { BadgeType } from "../../src/badge";
  import * as project from "../../src/project";

  import { Flex } from "../Primitive";
  import List from "./List.svelte";
  import ProjectCard from "./ProjectCard.svelte";
  import Stats from "./Stats.svelte";

  export let projects: project.Project[];
  export let urn: string;

  const projectCardProps = (project: project.Project) => ({
    title: project.metadata.name,
    description: project.metadata.description,
    showRegisteredBadge: !!project.registration,
    badge:
      (project.metadata.maintainers.includes(urn) && BadgeType.Maintainer) ||
      undefined,
  });
</script>

<List
  dataCy="project-list"
  items={projects}
  on:select
  let:item={project}
  style="margin: 0 auto;">
  <Flex
    style="flex: 1; padding: 1.375rem 1.5rem;"
    dataCy={`project-list-entry-${project.metadata.name}`}>
    <div slot="left">
      <ProjectCard {...projectCardProps(project)} />
    </div>
    <div slot="right" style="display: flex; align-items: center;">
      {#if project.stats}
        <Stats
          branches={project.stats.branches}
          commits={project.stats.commits}
          contributors={project.stats.contributors} />
      {/if}
    </div>
  </Flex>
</List>
