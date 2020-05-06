<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { projects as projectStore } from "../../src/project.ts";

  import { Icon, Text } from "../../DesignSystem/Primitive";
  import { ProjectList, Remote } from "../../DesignSystem/Component";

  const session = getContext("session");

  const select = (event) => {
    const project = event.detail;
    push(path.projectSource(project.id));
  };

  const contextMenuItems = (projectId, session) => {
    if (session.identity.registered) {
      return [
        {
          title: "Register project",
          icon: Icon.Register,
          event: () =>
            push(path.registerExistingProject(projectId, session.identity.id)),
        },
      ];
    } else {
      return [
        {
          title: "Register project",
          icon: Icon.Register,
          disabled: true,
          tooltip:
            "To unlock project registration, register your own handle first.",
        },
      ];
    }
  };
</script>

<Remote store={projectStore} let:data={projects}>
  {#if projects.length > 0}
    <ProjectList
      {projects}
      contextMenuItems={(projectId) => contextMenuItems(projectId, session)}
      on:select={select} />
  {:else}{push(path.profileOnboard())}{/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
