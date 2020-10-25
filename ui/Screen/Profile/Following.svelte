<script lang="typescript">
  import { getContext } from "svelte";
  import { fade } from "svelte/transition";
  import { push } from "svelte-spa-router";

  import { FADE_DURATION } from "../../src/config";
  import { Variant as IllustrationVariant } from "../../src/illustration";
  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import {
    followingProjects,
    fetchFollowingProjects,
    fetchRequestedProjects,
    requestedProjects,
  } from "../../src/profile";
  import type { Project } from "../../src/project";
  import type { Session } from "../../src/session";

  import {
    EmptyState,
    Hoverable,
    List,
    ProjectList,
    Remote,
    ShareableIdentifier,
    TrackToggle,
  } from "../../DesignSystem/Component";

  const session: Session = getContext("session");
  const onCancel = (urn: string): void => {
    console.log("cancel search", urn);
  };
  const onSelect = ({ detail: project }: { detail: Project }) => {
    push(path.projectSource(project.id));
  };

  fetchFollowingProjects();
  fetchRequestedProjects();

  $: console.log($requestedProjects);
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .header {
    display: flex;
    margin: 1.5rem 3rem 0.5rem;
  }

  .undiscovered-project {
    padding: 1.5rem;
    flex: 1;
    min-height: 6rem;

    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .undiscovered-project:hover {
    background-color: var(--color-background);
  }
</style>

<div class="container">
  <Remote store={followingProjects} let:data={projects}>
    <ProjectList
      {projects}
      userUrn={session.identity.urn}
      on:select={onSelect} />

    <div slot="empty">
      <EmptyState
        text="You're not following any projects yet."
        illustration={IllustrationVariant.Horse}
        primaryActionText="Look for a project"
        on:primaryAction={() => {
          modal.toggle(path.search());
        }} />
    </div>
  </Remote>

  <Remote store={requestedProjects} let:data={requests}>
    <div out:fade|local={{ duration: FADE_DURATION }}>
      <div class="header">
        <p class="typo-text-bold">Still looking…&nbsp;</p>
        <p style="color: var(--color-foreground-level-6);">
          These projects haven't been found in your network yet or don't exist.
        </p>
      </div>
      <List items={requests} let:item={request}>
        <Hoverable let:hovering={hover} style="flex: 1;">
          <div
            class="undiscovered-project"
            out:fade|local={{ duration: FADE_DURATION }}>
            <div>
              <ShareableIdentifier urn={request.urn} />
            </div>
            {#if hover}
              <div transition:fade={{ duration: FADE_DURATION }}>
                <TrackToggle
                  expanded
                  warning
                  tracking={true}
                  on:untrack={() => onCancel(request.urn)} />
              </div>
            {/if}
          </div>
        </Hoverable>
      </List>
    </div>
  </Remote>
</div>
