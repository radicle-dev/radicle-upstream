<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";

  import { fetchList, projects as store } from "ui/src/project";
  import { showNotificationsForFailedProjects } from "ui/src/profile";
  import * as Session from "ui/src/session";
  import * as modal from "ui/src/modal";
  import * as router from "ui/src/router";

  import CreateProjectModal from "ui/App/CreateProjectModal.svelte";
  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";
  import Error from "./Error.svelte";
  import ProjectList from "./ProjectList.svelte";
  import Remote from "ui/App/Remote.svelte";

  const session = Session.unsealed();

  const create = () => {
    modal.toggle(CreateProjectModal);
  };
  const select = ({ detail: project }: { detail: Project }) =>
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });

  fetchList();
  showNotificationsForFailedProjects();
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} userUrn={session.identity.urn} on:select={select} />

  <div slot="empty">
    <EmptyState
      text="You don’t have any projects yet."
      primaryActionText="Start your first project"
      on:primaryAction={create} />
  </div>

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
