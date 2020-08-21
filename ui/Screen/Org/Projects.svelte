<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import { projects as store, fetchProjectList } from "../../src/org.ts";
  import * as path from "../../src/path.ts";

  import { Flex } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    EmptyState,
    Error,
    List,
    ProjectCard,
    Remote,
    Stats,
  } from "../../DesignSystem/Component";

  export let params = null;
  const session = getContext("session");

  const select = event => {
    const orgProject = event.detail;
    if (orgProject.maybeProject) {
      push(path.projectSource(orgProject.maybeProject.id));
    }
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

  const create = () => push(path.registerProject(params.id));
  const register = () => push(path.memberRegistration(params.id));

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
            <Stats
              branches={orgProject.maybeProject.stats.branches}
              commits={orgProject.maybeProject.stats.commits}
              contributors={orgProject.maybeProject.stats.contributors} />
          {/if}

          <AdditionalActionsDropdown
            headerTitle={orgProject.shareableEntityIdentifier} />
        </div>
      </Flex>
    </List>
  {:else if session.permissions.registerProject}
    <EmptyState
      illustration="tent"
      text="There's nothing here yet, get started by creating your first project
      or adding a member to your org."
      primaryActionText="Register a project"
      secondaryActionText="Or add a member"
      on:primaryAction={create}
      on:secondaryAction={register} />
  {:else}
    <EmptyState
      illustration="tent"
      text="Add a member to your org."
      primaryActionText="Add a member"
      on:primaryAction={register} />
  {/if}

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
