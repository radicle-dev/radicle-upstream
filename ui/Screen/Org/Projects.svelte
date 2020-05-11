<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { Text } from "../../DesignSystem/Primitive";
  import { ProjectList, Remote } from "../../DesignSystem/Component";
  import { orgs as orgsStore, fetchProjectList } from "../../src/org.ts";

  export let params = null;

  const select = (event) => {
    const project = event.detail;
    push(path.projectSource(project.maybeProjectId));
  };

  $: fetchProjectList({ id: params.id });
</script>

<Remote store={orgsStore} let:data={projects}>
  {#if projects.length > 0}
    <ProjectList
      projects={projects.map((project) => {
        return { id: project.name, registration: { org: '123' }, maybeProjectId: project.maybeProjectId, metadata: { name: project.name } };
      })}
      on:select={select} />
  {:else}{push(path.orgOnboard(params.id))}{/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
