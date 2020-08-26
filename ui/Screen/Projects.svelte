<script>
  import { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { BadgeType } from "../src/badge.ts";

  import { Flex } from "../DesignSystem/Primitive";
  import { List, ProjectCard, Stats } from "../DesignSystem/Component";

  export let projects = null;
  export let urn = null;

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  const projectCardProps = project => ({
    title: project.metadata.name,
    description: project.metadata.description,
    showRegisteredBadge: project.registration,
    badge: project.metadata.maintainers.includes(urn) && BadgeType.Maintainer,
  });
</script>

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
