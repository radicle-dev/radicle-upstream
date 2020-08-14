<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectsStore } from "../../src/project.ts";

  import { Icon } from "../../DesignSystem/Primitive";
  import {
    EmptyState,
    Error,
    List,
    ProjectListItem,
    Remote,
  } from "../../DesignSystem/Component";

  const session = getContext("session");

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  const contextMenuItems = (projectId, session) => [
    {
      title: "Register project",
      dataCy: "register-project",
      icon: Icon.Register,
      disabled: !session.permissions.registerProject,
      tooltip: session.permissions.registerProject
        ? null
        : "Register your handle to register a project.",
      event: () =>
        session.permissions.registerProject &&
        push(
          path.registerExistingProject(projectId, session.identity.registered)
        ),
    },
  ];

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
      <ProjectListItem
        dataCy={`project-list-entry-${project.metadata.name}`}
        {...project}
        menuItems={contextMenuItems(project.id, session)} />
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
