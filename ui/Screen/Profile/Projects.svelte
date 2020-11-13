<script lang="typescript">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { fetchList, projects as store } from "../../src/project";
  import type { Project } from "../../src/project";
  import { showNotificationsForFailedProjects } from "../../src/profile";
  import type { UnsealedSession } from "../../src/session";

  import {
    EmptyState,
    Error,
    ProjectList,
    Remote,
  } from "../../DesignSystem/Component";

  const session: UnsealedSession = getContext("session");

  const create = () => {
    modal.toggle(path.newProject());
  };
  const select = ({ detail: project }: { detail: Project }) =>
    push(path.projectSource(project.id));

  fetchList();
  showNotificationsForFailedProjects();
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} userUrn={session.identity.urn} on:select={select} />

  <div slot="empty">
    <EmptyState
      text="You don't have any projects yet."
      primaryActionText="Start your first project"
      on:primaryAction={create} />
  </div>

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
