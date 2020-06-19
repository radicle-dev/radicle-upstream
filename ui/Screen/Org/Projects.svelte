<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import { projects as store, fetchProjectList } from "../../src/org.ts";
  import * as path from "../../src/path.ts";

  import { Flex, Icon, Text } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    List,
    ProjectCard,
    Remote,
    Stats,
  } from "../../DesignSystem/Component";

  import Onboard from "./Onboard.svelte";

  export let params = null;
  const session = getContext("session");

  const select = event => {
    const orgProject = event.detail;
    if (orgProject.maybeProject) {
      push(path.projectSource(orgProject.maybeProject.id));
    }
  };

  const statsProps = stats => {
    return [
      { icon: Icon.Commit, count: stats.commits },
      { icon: Icon.Branch, count: stats.branches },
      { icon: Icon.Member, count: stats.contributors },
    ];
  };

  const projectCardProps = orgProject => {
    if (orgProject.maybeProject) {
      return {
        title: orgProject.name,
        description: orgProject.maybeProject.metadata.description,
        showRegisteredBadge: true,
      };
    } else {
      return {
        title: orgProject.name,
        showRegisteredBadge: true,
      };
    }
  };

  $: fetchProjectList({ id: params.id });
</script>

<Remote {store} let:data={orgProjects}>
  {#if orgProjects.length > 0}
    <List
      dataCy="project-list"
      items={orgProjects}
      on:select={select}
      let:item={orgProject}
      style="margin: 0 auto;">
      <!-- TODO(julien): what should the registered but no coco metadata
        state look like visually? -->
      <Flex
        style="flex: 1; padding: 22px 15px 26px 12px;"
        dataCy={`project-${orgProject.name}`}>
        <div slot="left">
          <ProjectCard {...projectCardProps(orgProject)} />
        </div>

        <div slot="right" style="display: flex; align-items: center;">
          {#if orgProject.maybeProject}
            <Stats stats={statsProps(orgProject.maybeProject.stats)} />
          {/if}

          <AdditionalActionsDropdown
            headerTitle={orgProject.shareableEntityIdentifier} />
        </div>
      </Flex>
    </List>
  {:else}
    <Onboard
      orgId={params.id}
      registerProjectPermission={session.permissions.registerProject} />
  {/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
