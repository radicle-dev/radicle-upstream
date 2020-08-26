<script>
  import { Icon } from "../../DesignSystem/Primitive";

  export let title = null;
</script>

<style>
  .project-avatar {
    display: flex;
    align-items: center;
  }
</style>

<div data-cy="project-avatar" class="project-avatar">
  <Icon.Box />
  <p class="typo-text-bold" style="white-space: nowrap; margin-left: 8px;">
    {title}
  </p>
</div>
