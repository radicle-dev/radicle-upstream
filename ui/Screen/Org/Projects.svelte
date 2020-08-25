<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import {
    projects as store,
    fetchProjectList,
    formatProjectMetadata,
  } from "../../src/org.ts";
  import * as path from "../../src/path.ts";
  import { Variant as IllustrationVariant } from "../../src/illustration.ts";

  import {
    EmptyState,
    Error,
    List,
    ProjectListItem,
    Remote,
  } from "../../DesignSystem/Component";

  export let params = null;
  const session = getContext("session");

  const select = event => {
    const orgProject = event.detail;
    if (orgProject.maybeProject) {
      push(path.projectSource(orgProject.maybeProject.id));
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
      <ProjectListItem
        dataCy={`project-${orgProject.name}`}
        metadata={formatProjectMetadata(orgProject)}
        stats={orgProject.maybeProject && orgProject.maybeProject.stats}
        registration={true}
        shareableEntityIdentifier={orgProject.shareableEntityIdentifier} />
    </List>
  {:else if session.permissions.registerProject}
    <EmptyState
      illustration={IllustrationVariant.Tent}
      text="There's nothing here yet, get started by creating your first project
      or adding a member to your org."
      primaryActionText="Register a project"
      secondaryActionText="Or add a member"
      on:primaryAction={create}
      on:secondaryAction={register} />
  {:else}
    <EmptyState
      illustration={IllustrationVariant.Tent}
      text="Add a member to your org."
      primaryActionText="Add a member"
      on:primaryAction={register} />
  {/if}

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
