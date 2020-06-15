<script>
  import { Icon, Text, Title } from "../Primitive";

  export let title = null;
  export let description = null;
  export let showRegisteredBadge = false;
</script>

<style>
  .project-card {
    display: flex;
    width: 100%;
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
    margin-left: 9px;
  }
</style>

<div class="project-card">
  <div class="container">
    <div class="title-row">
      <Title>{title}</Title>
      {#if showRegisteredBadge}
        <div data-cy="registered" class="registered">
          <Icon.Verified
            style="fill: var(--color-primary); position: relative; bottom: -3px;" />
        </div>
      {/if}
    </div>
    <Text
      style="color: var(--color-foreground-level-5); white-space: nowrap;
      overflow: hidden; text-overflow: ellipsis;">
      {#if description}{description}{/if}
    </Text>
  </div>
</div>
