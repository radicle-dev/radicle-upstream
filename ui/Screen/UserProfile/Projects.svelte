<script lang="typescript">
  import * as router from "ui/src/router";

  import type { Project } from "../../src/project";
  import { fetchProjects, projects as store } from "../../src/userProfile";

  import { Error, ProjectList, Remote } from "../../DesignSystem/Component";

  export let urn: string;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      urn: project.urn,
      activeTab: "files",
      commitHash: null,
    });
  };

  fetchProjects(urn);
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} userUrn={urn} on:select={select} />

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
