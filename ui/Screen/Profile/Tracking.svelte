<script>
  import { fade } from "svelte/transition";
  import {
    Hoverable,
    List,
    ProjectList,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";

  import { FADE_DURATION } from "../../src/config.ts";

  const projects = [
    {
      metadata: {
        name: "obediah",
        description:
          "Obediah Hinton is one of the village elders in a small village named Butcher Creek.",
        maintainers: [],
      },
      registration: true,
      shareableEntityIdentifier: "obediah@38590438594",
    },
    {
      metadata: {
        name: "lemuel",
        description: "Lemuel is a villager from Butcher Creek.",
        maintainers: [],
      },
      registration: false,
      shareableEntityIdentifier: "lemuel@38590438594",
      maintainers: [],
    },
  ];

  const untracked = [
    {
      urn: "@hyndb5gs95gwtsf37tncz4ag3wqrg4ejw3qqga6x1srw9jp8jw59d6.git",
      metadata: {
        name: "snickers",
      },
    },
    {
      urn: "@hwd1yren6nte7ofh1sijz3tgc31cdmfb7zg7ya7gfgzwhhzgau8u13hkkjw.git",
      metadata: {
        name: "marsbar",
      },
    },
    {
      urn: "@hwd1yren6nte7ofh1sijz3tgc31cdmfb7zg7ya7gfgzwhhzgau8u13hkkjw.git",
      metadata: {
        name: "nougati",
      },
    },
  ];
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
  <ProjectList {projects} />
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
