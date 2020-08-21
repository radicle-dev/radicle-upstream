<script>
  import { fade } from "svelte/transition";
  import {
    Hoverable,
    List,
    ProjectListItem,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";

  import { fadeDuration } from "../../src/animation.ts";

  const projects = [
    {
      metadata: {
        name: "obediah",
        description:
          "Obediah Hinton is one of the village elders in a small village named Butcher Creek.",
      },
      registration: true,
      shareableEntityIdentifier: "obediah@38590438594",
    },
    {
      metadata: {
        name: "lemuel",
        description: "Lemuel is a villager from Butcher Creek.",
      },
      registration: false,
      shareableEntityIdentifier: "lemuel@38590438594",
    },
  ];

  const untracked = [
    { urn: "hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k" },
    { urn: "fjkldasjfkdlsajfio943we859043ikjioclesdjf" },
    { urn: "fjkldasjfkdlsajfio39409340390we859043ikjioclesdjf" },
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
    margin: 0.5rem 3rem;
  }

  .undiscovered-project {
    padding: 1rem;
    flex: 1;
    min-height: 4.5rem;

    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .undiscovered-project:hover {
    background-color: var(--color-background);
  }
</style>

<div class="container">
  <List items={projects} let:item={project} style="margin-bottom: 1.5rem;">
    <ProjectListItem
      metadata={project.metadata}
      registration={project.registration}
      shareableEntityIdentifier={project.shareableEntityIdentifier}
      stats={{ branches: 2, commits: 4, contributors: 8 }} />
  </List>

  {#if untracked.length}
    <div out:fade|local={{ duration: fadeDuration }}>
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
            out:fade|local={{ duration: fadeDuration }}>
            <Urn urn={project.urn} />
            {#if hover}
              <div transition:fade={{ duration: fadeDuration }}>
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
