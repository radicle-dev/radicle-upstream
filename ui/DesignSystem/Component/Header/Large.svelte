<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import * as org from "../../../src/org";
  import * as identity from "../../../src/identity";

  import { Avatar, Button, Icon, Title, Text, Numeric } from "../../Primitive";
  import Copyable from "../Copyable.svelte";

  const dispatch = createEventDispatcher();

  export let style = "";
  export let entity: org.Org | identity.Identity;

  const onRegisterHandle = () => {
    dispatch("registerHandle");
  };

  const isOrg = (entity: org.Org | identity.Identity): entity is org.Org =>
    (entity as org.Org).members !== undefined;
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
        variant={isOrg(entity) ? 'square' : 'circle'}
        avatarFallback={entity.avatarFallback} />

      <div class="metadata">
        <div class="user">
          <Title
            dataCy="entity-name"
            variant="huge"
            style="display: flex; align-items: center;">
            {#if isOrg(entity)}
              {entity.id}
            {:else if entity.registered}
              {entity.registered}
            {:else}
              {entity.metadata.handle}
              <Button
                variant="outline"
                style="margin-left: 12px;"
                on:click={() => onRegisterHandle()}>
                Register handle
              </Button>
            {/if}
          </Title>

          {#if isOrg(entity) || entity.registered}
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
          <Copyable>
            <Numeric
              variant="small"
              style="max-width: 20rem; white-space: nowrap; overflow: hidden;
              text-overflow: ellipsis; margin-right: 8px;">
              {entity.shareableEntityIdentifier}
            </Numeric>
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
