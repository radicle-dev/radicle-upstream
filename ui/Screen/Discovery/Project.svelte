<script>
  import {
    Avatar,
    Code,
    Icon,
    Text,
    Title,
  } from "../../DesignSystem/Primitive";
  import { Copyable, Stats, TrackToggle } from "../../DesignSystem/Component";

  export let project = null;

  let showTrackButton = false;

  const toggleTrackButton = e => {
    showTrackButton = e.type === "mouseenter";
  };
</script>

<style>
  .container {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 4px;
    padding: 32px;
    overflow: hidden;
  }

  .container:hover {
    border: 2px solid var(--color-foreground-level-3);
    background: var(--color-foreground-level-1);
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
    max-width: 180px;
    background: var(--color-foreground-level-2);
    margin-bottom: 16px;
    border-radius: 4px;
    padding: 4px;
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
  on:mouseleave={toggleTrackButton}>
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
      <TrackToggle peerCount="666" style="z-index: 10;" />
    {/if}

  </div>
  <div class="shareableEntityIdentifier">
    <Copyable>
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
    <Avatar avatarFallback={project.maintainers[0].avatar} />
  </div>
</div>
