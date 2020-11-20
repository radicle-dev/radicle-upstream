<script lang="typescript">
  import type { EmojiAvatar } from "../../../src/avatar";
  import type { Stats } from "../../../src/project";

  import { Avatar, Icon } from "../../Primitive";
  import ShareableIdentifier from "../ShareableIdentifier.svelte";
  import PeerId from "../PeerId.svelte";

  export let name: string;
  export let urn: string | null = null;
  export let peerId: string | null = null;
  export let description: string = "";

  export let avatarFallback: EmojiAvatar | undefined = undefined;
  export let avatarShape: "circle" | "square" = "circle";

  export let stats: Stats | undefined = undefined;
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
    min-width: 16rem;
    margin-right: 1.5rem;
  }

  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }

  .description {
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--color-foreground-level-6);
  }

  .project-stats {
    display: flex;
    margin-top: 0.5rem;
  }

  .project-stat-item {
    display: flex;
    color: var(--color-foreground-level-6);
    margin-right: 1rem;
  }

  .project-stat-item p {
    margin-left: 0.5rem;
    white-space: nowrap;
  }

  .project-stat-separator {
    display: flex;
    color: var(--color-foreground-level-3);
    margin-right: 1rem;
  }

  .banner-action {
    display: flex;
    align-items: center;
  }
</style>

<div data-cy="header" class="banner">
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
        <h1 data-cy="entity-name" class="typo-overflow-ellipsis" title={name}>
          {name}
        </h1>
        {#if urn}
          <ShareableIdentifier {urn} style="margin-top: 0.5rem;" />
        {:else if peerId}
          <PeerId {peerId} style="margin-top: 0.5rem;" />
        {/if}
        {#if description.length > 0}
          <p class="description typo-overflow-ellipsis" title={description}>
            {description}
          </p>
        {/if}
        {#if stats}
          <div class="project-stats" data-cy="project-stats">
            {#if stats.branches > 0}
              <div class="project-stat-item">
                <Icon.Branch />
                <p>
                  {stats.branches === 1 ? `1 Branch` : `${stats.branches} Branches`}
                </p>
              </div>
            {/if}
            {#if stats.branches > 0 && stats.contributors > 0}
              <span class="typo-mono-bold project-stat-separator">•</span>
            {/if}
            {#if stats.contributors > 0}
              <div class="project-stat-item">
                <Icon.User />
                <p>
                  {stats.contributors === 1 ? `1 Contributor` : `${stats.contributors} Contributors`}
                </p>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
    <div class="banner-action">
      <slot name="top" />
    </div>
  </div>
</div>
