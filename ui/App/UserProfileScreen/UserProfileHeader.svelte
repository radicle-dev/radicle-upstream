<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as identity from "ui/src/identity";
  import { Avatar, CopyableIdentifier } from "ui/DesignSystem";

  export let identityMetadata: identity.Metadata;
  export let urn: string;
  export let deviceIds: string[];
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
  .row {
    display: flex;
    margin-top: 0.5rem;
  }
</style>

<Avatar
  style="margin-right: 32px"
  size="huge"
  kind={{ type: "userEmoji", uniqueIdentifier: urn }} />

<div class="metadata">
  <h1
    data-cy="entity-name"
    class="typo-overflow-ellipsis"
    title={identityMetadata.handle}>
    {identityMetadata.handle}
  </h1>

  {#if identityMetadata.ethereum?.address}
    <div class="row" title={identityMetadata.ethereum?.address}>
      <CopyableIdentifier
        value={identityMetadata.ethereum?.address}
        kind="ethAddress" />
    </div>
  {/if}
  {#each deviceIds as deviceId}
    <div class="row" title={deviceId}>
      <CopyableIdentifier value={deviceId} kind="deviceId" />
    </div>
  {/each}
</div>
