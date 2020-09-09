<script>
  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import {
    Stats,
    TrackToggle,
    Tooltip,
    Urn,
  } from "../../DesignSystem/Component";

  export let project = null;

  let showTrackButton = project.tracked;

  const toggleTrackButton = e => {
    showTrackButton = project.tracked || e.type === "mouseenter";
  };
</script>

<style>
  .container {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 4px;
    padding: 24px;
    overflow: hidden;
    cursor: pointer;
  }

  .container:hover {
    background: var(--color-foreground-level-1);
    border-color: var(--color-foreground-level-3);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 40px;
  }

  .title {
    display: flex;
    flex: 1;
    min-width: 0;
    margin-right: 20px;
  }

  .shareable-entity-identifier {
    margin-bottom: 16px;
    display: flex;
    justify-content: flex-start;
  }

  .description {
    height: 100px;
    overflow: hidden;
  }

  .registered {
    display: flex;
    margin-left: 5px;
  }

  .bottom {
    display: flex;
    justify-content: space-between;
  }
</style>

<div
  class="container"
  on:mouseenter={toggleTrackButton}
  on:mouseleave={toggleTrackButton}
  data-cy="project-card">
  <div class="header">
    <div class="title">
      {#if project.registration}
        <h3 style="color: var(--color-foreground-level-4);">
          {project.domain}
        </h3>
        <h3 class="typo-overflow-ellipsis">
          &nbsp;{`/ ${project.metadata.name}`}
        </h3>
        <div class="registered">
          <Icon.RegisteredSmall
            style="fill: var(--color-primary); position: relative; bottom: -5px;" />
        </div>
      {:else}
        <h3 class="typo-overflow-ellipsis">{project.metadata.name}</h3>
      {/if}
    </div>

    {#if showTrackButton}
      <TrackToggle style="z-index: 10;" tracking={project.tracked} />
    {/if}
  </div>
  <div class="shareable-entity-identifier">
    <Urn
      urn={project.shareableEntityIdentifier}
      notificationText="The project ID was copied to your clipboard"
      truncate />
  </div>

  <div class="description">
    <p>{project.metadata.description}</p>
  </div>

  <div class="bottom">
    <Stats
      branches={project.stats.branches}
      commits={project.stats.commits}
      contributors={project.stats.contributors} />
    {#if project.maintainers && project.maintainers.length > 0}
      <Tooltip value={project.maintainers[0].handle}>
        <Avatar avatarFallback={project.maintainers[0].avatar} size="small" />
      </Tooltip>
    {/if}
  </div>
</div>
