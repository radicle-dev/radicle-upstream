<script>
  import { push } from "svelte-spa-router";
  import { getContext } from "svelte";

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

  console.log(session);
  const contextMenuItems = (projectId, session) => {
    const canRegister =
      session.permissions && session.permissions.registerProject;

    return [
      {
        title: "Register project",
        dataCy: "register-project",
        icon: Icon.Register,
        disabled: !canRegister,
        tooltip: canRegister
          ? null
          : "Register your handle to register a project.",
        event: () =>
          canRegister &&
          push(
            path.registerExistingProject(projectId, session.identity.registered)
          ),
      },
    ];
  };

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
      <ProjectListItem
        dataCy={`project-list-entry-${project.metadata.name}`}
        {...project}
        menuItems={contextMenuItems(project.id, session)} />
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
