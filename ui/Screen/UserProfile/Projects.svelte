<script lang="typescript">
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path";
  import type { Project } from "../../src/project";
  import { fetchProjects, projects as store } from "../../src/userProfile";

  import { Error, ProjectList, Remote } from "../../DesignSystem/Component";

  export let params: { urn: string };

  const select = ({ detail: project }: { detail: Project }) => {
    push(path.projectSource(project.id));
  };

  fetchProjects(params.urn);
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} urn={params.urn} on:select={select} />

  <div slot="error" let:error>
    <Error message={error && error.message} />
  </div>
</Remote>
