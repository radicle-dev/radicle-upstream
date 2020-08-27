<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { fetchList, projects as projectsStore } from "../../src/project.ts";

  import { Error, ProjectsList, Remote } from "../../DesignSystem/Component";

  export let params = null;

  const select = event => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  $: fetchList({ urn: params.urn });
</script>

<Remote store={projectsStore} let:data={projects}>
  <ProjectsList {projects} urn={params.urn} on:select={select} />

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
