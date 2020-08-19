<script>
  import {
    List,
    ProjectListItem,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";

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

    display: flex;
    align-items: center;
    justify-content: space-between;
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

  <div class="header">
    <p class="typo-text-bold">Still looking…&nbsp;</p>
    <p style="color: var(--color-foreground-level-6);">
      These projects haven't been found in your network yet or don't exist.
    </p>
  </div>

  <List
    items={[{ urn: 'hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k' }]}
    let:item={project}>
    <div class="undiscovered-project">
      <Urn urn={project.urn} />
      <TrackToggle tracking={true} />
    </div>
  </List>
</div>
