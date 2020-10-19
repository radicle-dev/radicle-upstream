<script>
  import { Avatar, Icon } from "../../DesignSystem/Primitive";

  export let user = null;
  export let title = null;
</script>

<style>
  .project-avatar {
    display: flex;
    align-items: center;
  }
</style>

<div data-cy="project-avatar" class="project-avatar">
  {#if user}
    <Avatar
      avatarFallback={user.avatarFallback}
      title={`${user.name} / ${title}`}
      variant="circle" />

    <Icon.Icon.RegisteredSmall
      style="margin-left: 6px; fill: var(--color-primary);" />
  {:else}
    <Icon.Box />
    <p class="typo-text-bold" style="white-space: nowrap; margin-left: 8px;">
      {title}
    </p>
  {/if}
</div>
