<script lang="typescript">
  import { push } from "ui/src/router";

  import type { Project } from "../../src/project";
  import { fetchProjects, projects as store } from "../../src/userProfile";

  import { Error, ProjectList, Remote } from "../../DesignSystem/Component";
  import ProjectScreen from "ui/Screen/Project.svelte";

  export let params: { urn: string };

  const select = ({ detail: project }: { detail: Project }) => {
    push({ component: ProjectScreen, props: { urn: project.urn } });
  };

  fetchProjects(params.urn);
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} userUrn={params.urn} on:select={select} />

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
