<script lang="typescript">
  import { isMaintainer } from "../../src/project";
  import type * as org from "ui/src/org";
  import { Icon } from "ui/DesignSystem/Primitive";

  import List from "./List.svelte";
  import ProjectCard from "./ProjectCard.svelte";
  import Stats from "./Stats.svelte";

  export let projects: org.ResolvedProject[];
  export let userUrn: string;

  const projectCardProps = (project: org.ResolvedProject) => ({
    title: project.project.metadata.name,
    description: project.project.metadata.description || "",
    showMaintainerBadge: isMaintainer(userUrn, project.project),
    anchored: project.type === "anchoredProject" ? true : false,
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

  .anchor-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
    color: var(--color-foreground-level-6);
  }

  .reset-cursor {
    cursor: default;
  }
</style>

<List
  dataCy="project-list"
  items={projects}
  on:select
  let:item={project}
  style="margin: 0 auto;">
  {#if project.type === "anchor"}
    <div
      class="reset-cursor list-item"
      data-cy={`project-list-entry-${project.anchor.id}`}>
      <div class="typo-text anchor-row">
        <Icon.At style="margin-right: 0.5rem;" />
        {project.anchor.projectId.replace("rad:git:", "")}
        <Icon.AnchorSmall
          style="fill: var(--color-primary); margin-left: 0.5rem;" />
      </div>
    </div>
  {:else}
    <div
      class="list-item"
      data-cy={`project-list-entry-${project.project.metadata.name}`}>
      <ProjectCard {...projectCardProps(project)} />
      {#if project.project.stats}
        <Stats
          branches={project.project.stats.branches}
          commits={project.project.stats.commits}
          contributors={project.project.stats.contributors} />
      {/if}
    </div>
  {/if}
</List>
