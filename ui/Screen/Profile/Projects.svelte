<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectsStore } from "../../src/project.ts";

  import {
    EmptyState,
    Error,
    ProjectList,
    Remote,
  } from "../../DesignSystem/Component";

  const create = () => push(path.createProject());

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

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
    <Error message={error.message} />
  </div>
</Remote>
