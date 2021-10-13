<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import {
    Avatar,
    Copyable,
    CopyableIdentifier,
    Icon,
    format,
  } from "ui/DesignSystem";

  import * as ensResolver from "ui/src/org/ensResolver";

  export let registration: ensResolver.Registration | undefined = undefined;
  export let orgAddress: string;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
    white-space: nowrap;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .name {
    margin-bottom: 0.5rem;
  }
  .domain {
    color: var(--color-foreground-level-4);
  }
</style>

<div style="display: flex">
  <Avatar
    style="margin-right: 2rem;"
    size="huge"
    kind={registration?.avatar
      ? { type: "orgImage", url: registration.avatar }
      : { type: "orgEmoji", uniqueIdentifier: orgAddress }} />

  <div class="metadata">
    {#if registration?.domain}
      <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
        {registration.domain.replace(`.${ensResolver.DOMAIN}`, "")}<span
          class="domain">.{ensResolver.DOMAIN}</span>
      </h1>

      <div style="display: flex; gap: 1rem;">
        <div>
          <div class="row">
            <Icon.Ethereum />
            <CopyableIdentifier
              value={orgAddress}
              kind="ethAddress"
              name="org address"
              showIcon={false} />
          </div>
        </div>
      </div>
    {:else}
      <Copyable
        name="org address"
        clipboardContent={orgAddress}
        tooltipStyle="width: fit-content;">
        <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
          {format.shortEthAddress(orgAddress)}
        </h1>
      </Copyable>
    {/if}
  </div>
</div>
