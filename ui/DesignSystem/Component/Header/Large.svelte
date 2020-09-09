<script>
  import { Avatar, Icon } from "../../Primitive";
  import Urn from "../Urn.svelte";

  export let style = null;
  export let entity = null;
  export let variant = null; // profile | project | org

  let name;
  if (variant === "profile") {
    if (entity.registered) {
      name = entity.registered;
    } else {
      name = entity.metadata.handle;
    }
  } else if (variant === "org") {
    name = entity.id;
  } else if (variant === "project") {
    name = entity.metadata.name;
  }
</script>

<style>
  .header {
    display: flex;
    flex-direction: column;
    width: calc(100vw - var(--sidebar-width));
    height: var(--bigheader-height);
    left: var(--sidebar-width);
    z-index: 2;
  }
  .banner {
    height: 200px;
    background-color: var(--color-foreground-level-1);
  }

  .banner-content {
    display: flex;
    justify-content: space-between;
    max-width: var(--content-max-width);
    width: 100%;
    margin: 0 auto;
    padding: 40px var(--content-padding);
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
  }

  .user {
    display: flex;
    align-items: center;
  }

  .shareable-entity-identifier {
    display: flex;
    align-items: center;
    padding-top: 4px;
  }

  .project-stats {
    height: 2.5rem;
    margin: 0.5rem 0;
    display: flex;
  }

  .project-stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem 0;
    margin-right: 1rem;
  }

  .stat {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    border-radius: 0.75rem;
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
    background-color: var(--color-background);
  }

  .banner-action {
    display: flex;
    align-items: center;
  }
</style>

<div data-cy="header" class="header" {style}>
  <div class="banner">
    <div class="banner-content">
      <div class="left">
        {#if variant !== 'project'}
          <Avatar
            style="margin-right: 32px"
            size="huge"
            variant={variant === 'profile' ? 'circle' : 'square'}
            avatarFallback={entity.avatarFallback} />
        {/if}

        <div class="metadata">
          <div class="user">
            <h1
              data-cy="entity-name"
              style="display: flex; align-items: center;">
              {name}
            </h1>
            {#if variant === 'org' || entity.registered}
              <Icon.Registered
                dataCy="verified-badge"
                style="fill: var(--color-primary); margin-left: 6px;" />
            {/if}
          </div>
          <div class="shareable-entity-identifier">
            <Urn
              urn={entity.shareableEntityIdentifier}
              showCopyOnlyOnHover
              notificationText={`Radicle ID for ${name} copied to your clipboard.`} />
          </div>
          {#if variant === 'project' && entity.metadata.description}
            <p class="description">{entity.metadata.description}</p>
            <div class="project-stats">
              <div class="project-stat-item">
                <Icon.Branch />
                <p style="margin: 0 8px;">Branches</p>
                <span class="stat typo-mono-bold">{entity.stats.branches}</span>
              </div>
              <div class="project-stat-item">
                <Icon.User />
                <p style="margin: 0 8px;">Contributors</p>
                <span class="stat typo-mono-bold">
                  {entity.stats.contributors}
                </span>
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
  <div class="action-bar">
    <slot name="left" />
    <slot name="right" />
  </div>
</div>
