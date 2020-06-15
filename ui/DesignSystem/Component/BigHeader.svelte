<script>
  export let style = null;
  export let data = null;
  export let variant = null; // profile | org

  import { Avatar, Icon, Title, Text, Numeric } from "../Primitive";
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
    max-width: var(--content-max-width);
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
    max-width: var(--content-max-width);
    padding: 0 16px;
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
        variant={variant === 'profile' ? 'circle' : 'square'}
        avatarFallback={data.avatarFallback} />

      <div class="metadata">
        <div class="user">
          {#if variant === 'profile'}
            <Title variant="huge">
              {data.registered ? data.registered : data.metadata.handle}
            </Title>
          {:else if variant === 'org'}
            <Title variant="huge">{data.id}</Title>
          {/if}
          {#if variant === 'org' || data.registered}
            <Icon.Verified
              size="large"
              style="fill: var(--color-primary); margin-left: 6px;" />
          {/if}
        </div>
        <div class="id">
          <Text variant="tiny" style="margin-right: 4px;">Radicle ID</Text>
          <Numeric variant="small">{data.shareableEntityIdentifier}</Numeric>
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
