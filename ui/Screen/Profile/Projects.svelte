<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import ModalNewProject from "../../Modal/NewProject.svelte";

  import * as modal from "ui/src/modal";
  import { fetchList, projects as store } from "ui/src/project";
  import type { Project } from "ui/src/project";
  import { showNotificationsForFailedProjects } from "../../src/profile";
  import * as router from "ui/src/router";
  import * as sess from "ui/src/session";

  import { EmptyState, Error, ProjectList, Remote } from "ui/DesignSystem";

  const session = sess.getUnsealedFromContext();

  const create = () => {
    modal.toggle(ModalNewProject);
  };
  const select = ({ detail: project }: { detail: Project }) =>
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });

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
