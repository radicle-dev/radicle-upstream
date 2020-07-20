<script>
  import {
    Avatar,
    Code,
    Icon,
    Text,
    Title,
  } from "../../DesignSystem/Primitive";
  import {
    Copyable,
    Stats,
    TrackToggle,
    Tooltip,
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

  .shareableEntityIdentifier {
    background: var(--color-foreground-level-2);
    margin-bottom: 16px;
    border-radius: 4px;
    padding: 4px;
    overflow: hidden;
    max-width: 180px;
  }

  .description {
    height: 100px;
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
      <Title variant="large" style="color: var(--color-foreground-level-4);">
        {project.domain}
      </Title>
      <Title variant="large" truncate>&nbsp;{`/ ${project.name}`}</Title>
      <div class="registered">
        <Icon.Verified
          style="fill: var(--color-primary); position: relative; bottom: -5px;" />
      </div>
    </div>

    {#if showTrackButton}
      <TrackToggle style="z-index: 10;" tracking={project.tracked} />
    {/if}

  </div>
  <div class="shareableEntityIdentifier">
    <Copyable style="min-width: 0;">
      <Code
        variant="medium"
        style="color: var(--color-foreground-level-5); font-size: 14px;
        text-overflow: ellipsis; white-space: nowrap; overflow: hidden;">
        {project.shareableEntityIdentifier}
      </Code>
    </Copyable>
  </div>
  <!-- TODO(sos): middle-truncate shareableEntityID & show copy icon -->

  <div class="description">
    <Text>{project.description}</Text>
  </div>

  <div class="bottom">
    <Stats
      branches={project.stats.branches}
      commits={project.stats.commits}
      contributors={project.stats.contributors} />
    <Tooltip value={project.maintainers[0].handle}>
      <Avatar avatarFallback={project.maintainers[0].avatar} size="small" />
    </Tooltip>
  </div>
</div>
