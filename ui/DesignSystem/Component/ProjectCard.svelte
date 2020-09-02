<script lang="ts">
  import { Icon } from "../Primitive";
  import Badge from "../Component/Badge.svelte";
  import { BadgeType } from "../../src/badge";

  export let title: string;

  export let badge: BadgeType | undefined = undefined;
  export let description: string | undefined = undefined;
  export let showRegisteredBadge = false;
</script>

<style>
  .project-card {
    display: flex;
    width: 100%;
    max-height: 3.2rem;
  }

  .container {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    width: 10%;
  }

  .title-row {
    display: flex;
    margin-bottom: 2px;
    white-space: nowrap;
  }

  .registered {
    display: flex;
  }

  .desc {
    color: var(--color-foreground-level-5);
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>

<div class="project-card">
  <div class="container">
    <div class="title-row">
      <p class="typo-text-bold">{title}</p>
      {#if showRegisteredBadge}
        <div data-cy="registered" class="registered">
          <Icon.RegisteredSmall style="fill: var(--color-primary);" />
        </div>
      {/if}
      {#if badge}
        <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
      {/if}
    </div>
    <p class="desc">
      {#if description}{description}{/if}
    </p>
  </div>
</div>
