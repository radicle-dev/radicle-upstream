<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { projects as projectsStore } from "../../src/project.ts";

  import { EmptyState, Error, Remote } from "../../DesignSystem/Component";

  import Projects from "../Projects.svelte";

  const create = () => push(path.createProject());

  const session = getContext("session");
</script>

<Remote store={projectsStore} let:data={projects}>
  {#if projects.length > 0}
    <Projects {projects} urn={session.identity.urn} />
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
