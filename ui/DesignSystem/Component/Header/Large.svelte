<script lang="ts">
  import type { EmojiAvatar } from "../../../src/avatar";

  import type { Identity } from "../../../src/identity";
  import type { Org } from "../../../src/org";

  import { Avatar, Icon } from "../../Primitive";
  import Urn from "../Urn.svelte";

  export let name: string;
  export let urn: string;
  export let description: string;
  export let registered: boolean = false;

  export let avatarFallback: EmojiAvatar;
  export let avatarShape: "circle" | "square" = "circle";

  export let stats: { string: string };

  export let style = "";

  let scrollY = 0;
  let headerHeight;
</script>

<style>
  .banner {
    height: 12.5rem;
    background-color: var(--color-foreground-level-1);
    display: flex;
  }

  .banner-content {
    display: flex;
    justify-content: space-between;
    max-width: var(--content-max-width);
    width: 100%;
    margin: 0 auto;
    padding: 0 var(--content-padding);
  }

  .left {
    display: flex;
  }

  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
  }

  .description {
    margin-top: 1rem;
    color: var(--color-foreground-level-6);

    height: 1.5rem; /* 1 line */
    max-width: 60vw;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
  .user {
    display: flex;
    align-items: center;
  }

  .shareable-entity-identifier {
    display: flex;
    align-items: center;
    padding-top: 0.25rem;
  }

  .project-stats {
    display: flex;
    margin-top: 1rem;
  }

  .project-stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
    margin-right: 1rem;
  }

  .project-stat-separator {
    display: flex;
    color: var(--color-foreground-level-3);
    margin-right: 1rem;
  }

  .action-bar-wrapper {
    background-color: var(--color-background);
    position: sticky;
    top: 0;
  }

  .elevation {
    box-shadow: var(--elevation-low);
  }

  .action-bar {
    display: flex;
    justify-content: space-between;
    height: var(--topbar-height);
    width: 100%;
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    align-items: center;
  }

  .banner-action {
    display: flex;
    align-items: center;
  }
</style>

<svelte:window bind:scrollY />

<div data-cy="header" class="banner" bind:clientHeight={headerHeight}>
  <div class="banner-content">
    <div class="left">
      {#if avatarFallback}
        <Avatar
          style="margin-right: 32px"
          size="huge"
          variant={avatarShape}
          {avatarFallback} />
      {/if}

      <div class="metadata">
        <div class="user">
          <h1 data-cy="entity-name" style="display: flex; align-items: center;">
            {name}
          </h1>
          {#if registered}
            <Icon.Registered
              dataCy="verified-badge"
              style="fill: var(--color-primary); margin-left: 6px;" />
          {/if}
        </div>
        <div class="shareable-entity-identifier">
          <Urn
            {urn}
            showCopyOnlyOnHover
            notificationText={`Radicle ID for ${name} copied to your clipboard.`} />
        </div>
        {#if description}
          <p class="description">{description}</p>
        {/if}
        {#if stats}
          <div class="project-stats" data-cy="project-stats">
            <div class="project-stat-item">
              <Icon.Branch />
              <p style="margin-left: 0.5rem;">{stats.branches} Branches</p>
            </div>
            <span class="typo-mono-bold project-stat-separator">•</span>
            <div class="project-stat-item">
              <Icon.User />
              <p style="margin-left: 0.5rem;">
                {stats.contributors} Contributors
              </p>
            </div>
          </div>
        {/if}
      </div>
    </div>
    <div class="banner-action">
      <slot name="top" />
    </div>
  </div>
</div>
<div class="action-bar-wrapper" class:elevation={scrollY > headerHeight}>
  <div class="action-bar">
    <slot name="left" />
    <slot name="right" />
  </div>
</div>
