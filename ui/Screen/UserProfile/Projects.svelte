<script>
  import { fetchList, projects as projectsStore } from "../../src/project.ts";

  import { Error, Remote } from "../../DesignSystem/Component";

  import Projects from "../Projects.svelte";

  export let params = null;

  $: fetchList({ urn: params.urn });
</script>

<Remote store={projectsStore} let:data={projects}>
  <Projects {projects} />

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
