<script lang="typescript">
  import { fade } from "svelte/transition";
  import { push } from "svelte-spa-router";

  import { FADE_DURATION } from "../../src/config";
  import { Variant as IllustrationVariant } from "../../src/illustration";
  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { tracked, fetchTracked } from "../../src/project";
  import type { Project } from "../../src/project";

  import {
    EmptyState,
    Hoverable,
    List,
    ProjectList,
    Remote,
    ShareableIdentifier,
    TrackToggle,
  } from "../../DesignSystem/Component";

  const onSelect = (project: Project) => {
    push(path.projectSource(project.id));
  };

  fetchTracked();
  $: console.log($tracked);

  // const untracked = [
  //   {
  //     urn: "rad:git:hwd1yrermy9kfw69u4obq9wcej1mbx1qn4byg4u35hd61c5qmnwxd5at8to",
  //   },
  //   {
  //     urn: "rad:git:hwd1yrermy9kfw69u4obq9wcej1mbx1qn4byg4u35hd61c5qmnwxd5at8to",
  //   },
  //   {
  //     urn: "rad:git:hwd1yrermy9kfw69u4obq9wcej1mbx1qn4byg4u35hd61c5qmnwxd5at8to",
  //   },
  // ];

  const untracked: { urn: string }[] = [];
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

  .project-name {
    color: var(--color-foreground-level-6);
    padding-bottom: 0.375rem;
  }
</style>

<div class="container">
  <Remote store={tracked} let:data={projects}>
    {#if projects.length > 0}
      <ProjectList {projects} on:select={ev => onSelect(ev.detail)} />
    {:else}
      <EmptyState
        text="You're not following any projects yet."
        illustration={IllustrationVariant.Horse}
        primaryActionText="Look for a project"
        on:primaryAction={() => {
          modal.toggle(path.search());
        }} />
    {/if}
  </Remote>

  {#if untracked.length}
    <div out:fade|local={{ duration: FADE_DURATION }}>
      <div class="header">
        <p class="typo-text-bold">Still looking…&nbsp;</p>
        <p style="color: var(--color-foreground-level-6);">
          These projects haven't been found in your network yet or don't exist.
        </p>
      </div>
      <List items={untracked} let:item={project}>
        <Hoverable let:hovering={hover} style="flex: 1;">
          <div
            class="undiscovered-project"
            out:fade|local={{ duration: FADE_DURATION }}>
            <div>
              <ShareableIdentifier urn={project.urn} />
            </div>
            {#if hover}
              <div transition:fade={{ duration: FADE_DURATION }}>
                <TrackToggle
                  tracking={true}
                  expanded
                  warning
                  on:untrack={() => console.log(`untrack ${project.urn}`)} />
              </div>
            {/if}
          </div>
        </Hoverable>
      </List>
    </div>
  {/if}
</div>
