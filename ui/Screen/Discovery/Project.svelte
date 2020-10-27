<script>
  import { Avatar } from "../../DesignSystem/Primitive";
  import {
    FollowButton,
    ShareableIdentifier,
    Stats,
    Tooltip,
  } from "../../DesignSystem/Component";

  export let project = null;

  let showFollowButton = project.tracked;

  const toggleFollowButton = e => {
    showFollowButton = project.tracked || e.type === "mouseenter";
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

  .description {
    height: 100px;
    overflow: hidden;
  }

  .bottom {
    display: flex;
    justify-content: space-between;
  }
</style>

<div
  class="container"
  on:mouseenter={toggleFollowButton}
  on:mouseleave={toggleFollowButton}
  data-cy="project-card">
  <div class="header">
    <div class="title">
      <h3 class="typo-overflow-ellipsis">{project.metadata.name}</h3>
    </div>

    {#if showFollowButton}
      <FollowButton style="z-index: 10;" following={project.tracked} />
    {/if}
  </div>

  <ShareableIdentifier urn={project.id} style="margin-bottom: 1rem;" />

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
