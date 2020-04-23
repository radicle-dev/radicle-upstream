<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { projectNameStore, projects } from "../../src/project.ts";

  import { Text } from "../../DesignSystem/Primitive";
  import {
    ProjectList,
    ProjectOnboard,
    Remote
  } from "../../DesignSystem/Component";

  const create = () => push(path.createProject());
  const register = () => push(path.registerUser());
  const select = event => {
    const project = event.detail;

    projectNameStore.set(project.metadata.name);
    push(path.projectSource(project.id));
  };
</script>

<Remote store={projects}>
  <div slot="success" let:data>
    {#if data.length > 0}
      <ProjectList projects={data} on:select={select} />
    {:else}
      <ProjectOnboard on:create={create} on:registe={register} />
    {/if}
  </div>

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
