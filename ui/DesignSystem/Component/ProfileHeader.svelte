<script>
  export let style = null;
  export let id = null;

  import { Avatar, Icon, Title, Text, Numeric } from "../Primitive";
</script>

<style>
  .header {
    display: flex;
    flex-direction: column;
    width: calc(100vw - var(--sidebar-width));
    height: 261px;
    left: var(--sidebar-width);
    z-index: 2;
  }
  .banner {
    height: 200px;
    background-color: var(--color-foreground-level-1);
  }

  .banner-content {
    display: flex;
    max-width: 1148px;
    width: 100%;
    margin: 0 auto;
    padding: 40px 16px;
  }

  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
  }

  .user {
    display: flex;
    align-items: center;
  }

  .id {
    display: flex;
    padding-top: 8px;
    color: var(--color-foreground-level-6);
  }

  .action-bar {
    display: flex;
    justify-content: space-between;
    height: 61px;
    width: 100%;
    max-width: 1148px;
    margin: 0 auto;
    align-items: center;
  }
</style>

<div data-cy="header" class="header" {style}>
  <div class="banner">
    <div class="banner-content">
      <Avatar
        style="margin-right: 32px"
        size="huge"
        variant="circle"
        avatarFallback={id.avatarFallback} />
      <div class="metadata">
        <div class="user">
          {#if id.registered}
            <Title variant="huge">{id.registered}</Title>
            <Icon.Verified
              size="large"
              style="fill: var(--color-primary); margin-left: 6px;" />
          {:else}
            <Title variant="huge">{id.metadata.handle}</Title>
          {/if}
        </div>
        <div class="id">
          <Text variant="tiny" style="margin-right: 4px;">Radicle ID</Text>
          <Numeric variant="small">{id.id}</Numeric>
        </div>
      </div>
    </div>
  </div>
  <div class="action-bar">
    <slot name="left" />
    <div class="right">
      <slot name="right" />
    </div>
  </div>
</div>
