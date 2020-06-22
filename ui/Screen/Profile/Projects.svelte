<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectStore } from "../../src/project.ts";

  import { Flex, Icon, Text } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    EmptyState,
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

  const create = () => push(path.createProject());
  const register = () => push(path.registerUser());
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
        dataCy={`project-list-entry-${project.metadata.name}`}>
        <div slot="left">
          <ProjectCard {...projectCardProps(project)} />
        </div>

        <div slot="right" style="display: flex; align-items: center;">
          <Stats stats={statsProps(project.stats)} />
          <AdditionalActionsDropdown
            dataCy="context-menu"
            headerTitle={project.shareableEntityIdentifier}
            menuItems={contextMenuItems(project.id, session)} />
        </div>
      </Flex>
    </List>
  {:else if session.permissions.registerHandle}
    <EmptyState
      text="There’s nothing here yet, get started by starting your first
      project."
      mainCtaText="Start your first project"
      mainDataCy="add-project-button"
      mainCtaAction={create}
      secondaryCtaText="Or register your handle"
      secondaryCtaAction={register}
      secondaryDataCy="register-handle-button" />
  {:else}
    <EmptyState
      text="There’s nothing here yet, get started by starting your first
      project."
      mainCtaText="Start your first project"
      mainDataCy="add-project-button"
      mainCtaAction={create} />
  {/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
