<script>
  import { push } from "svelte-spa-router";

  import { orgs as orgsStore, fetchProjectList } from "../../src/org.ts";
  import * as path from "../../src/path.ts";

  import { Flex, Icon, Text } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    List,
    ProjectCard,
    Remote,
    Stats,
  } from "../../DesignSystem/Component";

  export let params = null;

  const select = (event) => {
    const orgProject = event.detail;
    if (orgProject.maybeProject) {
      push(path.projectSource(orgProject.maybeProject.id));
    }
  };

  $: fetchProjectList({ id: params.id });

  const statsProps = (stats) => {
    return [
      { icon: Icon.Commit, count: stats.commits },
      { icon: Icon.Branch, count: stats.branches },
      { icon: Icon.Member, count: stats.contributors },
    ];
  };

  const projectCardProps = (orgProject) => {
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
</script>

<Remote store={orgsStore} let:data={orgProjects}>
  {#if orgProjects.length > 0}
    <List items={orgProjects} on:select={select} let:item={orgProject}>
      {#if orgProject.maybeProject}
        <Flex style="flex: 1">
          <div slot="left">
            <ProjectCard {...projectCardProps(orgProject)} />
          </div>

          <div slot="right" style="display: flex; align-items: center;">
            <Stats stats={statsProps(orgProject.maybeProject.stats)} />
            <AdditionalActionsDropdown headerTitle={orgProject.name} />
          </div>
        </Flex>
      {:else}
        <!-- TODO(julien): what should the registered but no coco metadata
        state look like visually? -->
        <Flex style="flex: 1">
          <div slot="left">
            <ProjectCard {...projectCardProps(orgProject)} />
          </div>
          <div slot="right" style="display: flex; align-items: center;">
            <AdditionalActionsDropdown headerTitle={orgProject.name} />
          </div>
        </Flex>
      {/if}
    </List>
  {:else}{push(path.orgOnboard(params.id))}{/if}

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
