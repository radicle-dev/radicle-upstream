<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectsStore } from "../../src/project.ts";
  import { BadgeType } from "../../src/badge.ts";

  import { Flex } from "../../DesignSystem/Primitive";
  import {
    EmptyState,
    Error,
    List,
    ProjectCard,
    Remote,
    Stats,
  } from "../../DesignSystem/Component";

  const session = getContext("session");

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  const projectCardProps = project => ({
    title: project.metadata.name,
    description: project.metadata.description,
    showRegisteredBadge: project.registration,
    badge:
      project.metadata.maintainers.includes(session.identity.urn) &&
      BadgeType.Maintainer,
  });

  const create = () => push(path.createProject());
</script>

<Remote store={projectsStore} let:data={projects}>
  {#if projects.length > 0}
    <List
      dataCy="project-list"
      items={projects}
      on:select={select}
      let:item={project}
      style="margin: 0 auto;">
      <Flex
        style="flex: 1; padding: 24px 16px 24px 24px;"
        dataCy={`project-list-entry-${project.metadata.name}`}>
        <div slot="left">
          <ProjectCard {...projectCardProps(project)} />
        </div>
        <div slot="right" style="display: flex; align-items: center;">
          <Stats
            branches={project.stats.branches}
            commits={project.stats.commits}
            contributors={project.stats.contributors} />
        </div>
      </Flex>
    </List>
  {:else}
    <EmptyState
      text="There’s nothing here yet, get started by starting your first
      project."
      primaryActionText="Create a new project"
      on:primaryAction={create} />
  {/if}

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
