<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectsStore } from "../../src/project.ts";

  import { Flex, Icon } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
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

  const contextMenuItems = (projectId, session) => {
    if (session.permissions.registerProject) {
      return [
        {
          title: "Register project",
          dataCy: "register-project",
          icon: Icon.Ledger,
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
          icon: Icon.Ledger,
          disabled: true,
          tooltip: "Register your handle to register a project.",
        },
      ];
    }
  };

  const projectCardProps = project => ({
    title: project.metadata.name,
    description: project.metadata.description,
    showRegisteredBadge: project.registration,
  });

  const create = () => push(path.createProject());
  const register = () => push(path.registerUser());
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
      primaryActionText="Start your first project"
      secondaryActionText="Or register your handle"
      on:primaryAction={create}
      on:secondaryAction={register} />
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
