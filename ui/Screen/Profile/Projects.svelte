<script lang="typescript">
  import { push } from "ui/vendor/svelte-spa-router";

  import ModalNewProject from "../../Modal/NewProject.svelte";

  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { fetchList, projects as store } from "../../src/project";
  import type { Project } from "../../src/project";
  import { showNotificationsForFailedProjects } from "../../src/profile";
  import * as sess from "../../src/session";

  import {
    EmptyState,
    Error,
    ProjectList,
    Remote,
  } from "../../DesignSystem/Component";

  const session = sess.getUnsealedFromContext();

  const create = () => {
    modal.toggle(ModalNewProject);
  };
  const select = ({ detail: project }: { detail: Project }) =>
    push(path.project(project.urn));

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
