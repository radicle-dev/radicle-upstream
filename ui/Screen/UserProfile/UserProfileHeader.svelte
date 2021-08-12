<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as proxyIdentity from "ui/src/proxy/identity";
  import { Avatar, Icon, StyledCopyable, PeerId } from "ui/DesignSystem";

  export let identityMetadata: proxyIdentity.Metadata;
  export let avatarFallback: proxyIdentity.Avatar;
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
  variant="circle"
  {avatarFallback} />

<div class="metadata">
  <h1
    data-cy="entity-name"
    class="typo-overflow-ellipsis"
    title={identityMetadata.handle}>
    {identityMetadata.handle}
  </h1>

  {#if identityMetadata.ethereum?.address}
    <div class="row" title={identityMetadata.ethereum?.address}>
      <Icon.Ethereum style="margin-right: 0.25rem;" />
      <StyledCopyable
        truncate
        expandable={false}
        value={identityMetadata.ethereum?.address} />
    </div>
  {/if}
  {#each deviceIds as deviceId}
    <div class="row" title={deviceId}>
      <PeerId truncate expandable={false} peerId={deviceId} />
    </div>
  {/each}
</div>
