<script>
  import { fade } from "svelte/transition";
  import {
    Hoverable,
    List,
    ProjectList,
    Remote,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";

  import { tracked, fetchTracked } from "../../src/project.ts";
  import { FADE_DURATION } from "../../src/config.ts";

  fetchTracked();
  $: console.log($tracked);

  // const untracked = [
  //   {
  //     urn: "@hyndb5gs95gwtsf37tncz4ag3wqrg4ejw3qqga6x1srw9jp8jw59d6.git",
  //     metadata: {
  //       name: "snickers",
  //     },
  //   },
  //   {
  //     urn: "@hwd1yren6nte7ofh1sijz3tgc31cdmfb7zg7ya7gfgzwhhzgau8u13hkkjw.git",
  //     metadata: {
  //       name: "marsbar",
  //     },
  //   },
  //   {
  //     urn: "@hwd1yren6nte7ofh1sijz3tgc31cdmfb7zg7ya7gfgzwhhzgau8u13hkkjw.git",
  //     metadata: {
  //       name: "nougati",
  //     },
  //   },
  // ];

  const untracked = [];
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
    min-height: 4.5rem;

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
      <ProjectList {projects} />
    {:else}
      <div
        style="width: 100%; color: var(--color-primary); display:flex; justify-content: center; flex-direction: column; align-items: center; margin-bottom: 4rem;">
        <h1>Nothing yet....</h1>
        <img
          src="https://media0.giphy.com/media/YaZgr3Nj9DDI4/giphy.gif?cid=ecf05e47fzjml7zt68cprbas3q92z07x5lyhi18ho5ba9vbd&rid=giphy.gif" />
      </div>
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
              <p class="project-name typo-text-bold">{project.metadata.name}</p>
              <Urn urn={project.urn} showCopyOnlyOnHover />
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
