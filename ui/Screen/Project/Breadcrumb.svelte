<script>
  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { IdentityAvatar } from "../../DesignSystem/Component";

  export let org = null;
  export let user = null;
  export let title = null;
</script>

<style>
  .project-avatar {
    display: flex;
    align-items: center;
  }
</style>

<div class="project-avatar">
  {#if user}
    <Avatar
      avatarFallback={user.avatarFallback}
      title={`${user.name} / ${title}`}
      variant="user" />

    <Icon.Badge style="margin-left: 6px; fill: var(--color-primary);" />
  {:else if org}
    <Avatar
      avatarFallback={org.avatarFallback}
      title={`${org.name} / ${title}`}
      variant="project" />

    <Icon.Badge style="margin-left: 6px; fill: var(--color-primary);" />
  {:else}
    <IdentityAvatar showTitle={true} titleSuffix={` / ${title}`} />
  {/if}
</div>
