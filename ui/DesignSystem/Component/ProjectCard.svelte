<script lang="ts">
  import { Icon } from "../Primitive";
  import Badge from "../Component/Badge.svelte";
  import { BadgeType } from "../../src/badge";

  export let title: string;

  export let description = "";
  export let showRegisteredBadge = false;
  export let showMaintainerBadge = false;
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
    white-space: nowrap;
  }

  .registered {
    display: flex;
  }

  .desc {
    color: var(--color-foreground-level-5);
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 0.125rem;
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
      {#if showMaintainerBadge}
        <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
      {/if}
    </div>
    {#if description.length > 0}
      <p class="desc">{description}</p>
    {/if}
  </div>
</div>
