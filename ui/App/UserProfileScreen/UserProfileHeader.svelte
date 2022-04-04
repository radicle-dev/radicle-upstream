<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as proxyIdentity from "proxy-client/identity";

  import Avatar from "design-system/Avatar.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let user: proxyIdentity.RemoteIdentity;
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
  kind={{ type: "userEmoji", uniqueIdentifier: user.urn }} />

<div class="metadata">
  <h1
    data-cy="entity-name"
    class="typo-overflow-ellipsis"
    title={user.metadata.handle}>
    {user.metadata.handle}
  </h1>

  {#if user.metadata.ethereum?.address}
    <div class="row" title={user.metadata.ethereum?.address}>
      <CopyableIdentifier
        value={user.metadata.ethereum?.address}
        kind="ethAddress" />
    </div>
  {/if}
  {#each user.peerIds as peerId}
    <div class="row" title={peerId}>
      <CopyableIdentifier value={peerId} kind="peerId" />
    </div>
  {/each}
</div>
