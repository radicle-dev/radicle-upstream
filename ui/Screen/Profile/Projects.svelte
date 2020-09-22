<script lang="ts">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { projects as projectsStore } from "../../src/project";
  import type { Project } from "../../src/project";

  import {
    EmptyState,
    Error,
    ProjectList,
    Remote,
  } from "../../DesignSystem/Component";

  const create = () => {
    modal.toggle(path.newProject());
  };

  const select = (event: { detail: Project }) =>
    push(path.projectSource(event.detail.id));

  const session = getContext("session");
</script>

<Remote store={projectsStore} let:data={projects}>
  {#if projects.length > 0}
    <ProjectList {projects} urn={session.identity.urn} on:select={select} />
  {:else}
    <EmptyState
      text="You don't have any projects yet."
      primaryActionText="Start your first project"
      on:primaryAction={create} />
  {/if}

  <div slot="error" let:error>
    <Error message={error && error.message} />
  </div>
</Remote>
