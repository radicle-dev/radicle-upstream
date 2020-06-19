<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectStore } from "../../src/project.ts";

  import { Button, Flex, Icon, Text } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    List,
    ProjectCard,
    Remote,
    Stats,
  } from "../../DesignSystem/Component";

  import Onboard from "./Onboard.svelte";

  let hover = false;

  const handleMouseenter = () => {
    hover = true;
  };

  const handleMouseleave = () => {
    hover = false;
  };

  const session = getContext("session");

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  const contextMenuItems = (projectId, session) => {
    if (session.permissions.registerProject) {
      return [
        {
          title: "Register project",
          dataCy: "register-project",
          icon: Icon.Register,
          event: () =>
            push(
              path.registerExistingProject(
                projectId,
                session.identity.registered
              )
            ),
        },
      ];
    } else {
      return [
        {
          title: "Register project",
          dataCy: "register-project",
          icon: Icon.Register,
          disabled: true,
          tooltip:
            "To unlock project registration, register your own handle first.",
        },
      ];
    }
  };

  const statsProps = stats => {
    return [
      { icon: Icon.Commit, count: stats.commits },
      { icon: Icon.Branch, count: stats.branches },
      { icon: Icon.Member, count: stats.contributors },
    ];
  };

  const projectCardProps = project => {
    return {
      title: project.metadata.name,
      description: project.metadata.description,
      showRegisteredBadge: project.registration,
    };
  };
</script>

<Remote store={projectStore} let:data={projects}>
  {#if projects.length > 0}
    <List
      dataCy="project-list"
      items={projects}
      on:select={select}
      let:item={project}
      style="margin: 0 auto;">
      <Flex
        style="flex: 1; padding: 24px 16px 24px 24px;"
        dataCy={`project-list-entry-${project.id}`}
        on:mouseenter={project.stats ? null : handleMouseenter}
        on:mouseleave={project.stats ? null : handleMouseleave}>
        <div slot="left">
          {#if project.metadata}
            <ProjectCard {...projectCardProps(project)} />
          {:else}
            <ProjectCard
              title="Unreplicated project"
              description={project.id} />
          {/if}
        </div>

        <div slot="right" style="display: flex; align-items: center;">
          {#if project.stats}
            <Stats stats={statsProps(project.stats)} />
          {:else if hover === true}
            <Button
              variant="secondary"
              icon={Icon.Peer}
              style="margin-right: 16px;">
              Track
            </Button>
          {/if}
          <AdditionalActionsDropdown
            dataCy="context-menu"
            headerTitle={project.shareableEntityIdentifier}
            menuItems={contextMenuItems(project.id, session)} />
        </div>
      </Flex>
    </List>
  {:else}
    <Onboard />
  {/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
