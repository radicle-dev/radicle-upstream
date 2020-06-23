<script>
  import { createEventDispatcher } from "svelte";
  import { Avatar, Button, Icon, Title, Text, Numeric } from "../Primitive";
  import { Copyable } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

  export let style = null;
  export let entity = null;
  export let variant = null; // profile | org

  const onRegisterAction = () => {
    dispatch("registerAction");
  };

  let copyIcon = Icon.Copy;

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 1000);
  };
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
    padding: 40px var(--content-padding);
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

  .shareable-entity-identifier {
    display: flex;
    padding-top: 8px;
    color: var(--color-foreground-level-6);
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
</style>

<div data-cy="header" class="header" {style}>
  <div class="banner">
    <div class="banner-content">
      <Avatar
        style="margin-right: 32px"
        size="huge"
        variant={variant === 'profile' ? 'circle' : 'square'}
        avatarFallback={entity.avatarFallback} />

      <div class="metadata">
        <div class="user">
          <Title dataCy="entity-name" variant="huge" style="display: flex;">
            {#if variant === 'profile' && entity.registered}
              @{entity.registered}
            {:else if variant === 'profile' && !entity.registered}
              {entity.metadata.handle}
              <Button
                variant="outline"
                style="margin-left: 12px;"
                on:click={() => onRegisterAction()}>
                Register handle
              </Button>
            {:else if variant === 'org'}{entity.id}{/if}
          </Title>
          {#if variant === 'org' || entity.registered}
            <Icon.Verified
              dataCy="verified-badge"
              size="large"
              style="fill: var(--color-primary); margin-left: 6px;" />
          {/if}
        </div>
        <div class="shareable-entity-identifier">
          <Text variant="tiny" style="margin-right: 4px; white-space: nowrap;">
            Radicle ID
          </Text>
          <Copyable {afterCopy} style="display: flex;">
            <Numeric
              variant="small"
              style="max-width: 20rem; white-space: nowrap; overflow: hidden;
              text-overflow: ellipsis;">
              {entity.shareableEntityIdentifier}
            </Numeric>
            <svelte:component
              this={copyIcon}
              size="small"
              style="margin-left: 8px; vertical-align: bottom;" />
          </Copyable>

        </div>
      </div>
    </div>
  </div>
  <div class="action-bar">
    <slot name="left" />
    <slot name="right" />
  </div>
</div>
