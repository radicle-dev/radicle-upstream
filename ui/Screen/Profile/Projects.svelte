<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { projectNameStore, projects as store } from "../../src/project.ts";

  import { Text } from "../../DesignSystem/Primitive";
  import { ProjectList, Remote } from "../../DesignSystem/Component";

  const select = event => {
    const project = event.detail;

    projectNameStore.set(project.metadata.name);
    push(path.projectSource(project.id));
  };
</script>

<Remote {store} let:data={projects}>
  {#if projects.length > 0}
    <ProjectList {projects} on:select={select} />
  {:else}{push(path.profileOnboard())}{/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
