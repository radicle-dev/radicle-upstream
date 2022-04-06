<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { User } from "ui/src/project";
  import { createEventDispatcher } from "svelte";

  import * as project from "ui/src/project";

  import TrackToggle from "design-system/TrackToggle.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";
  import UserBadge from "ui/App/SharedComponents/UserBadge.svelte";

  export let peer: User;
  export let projectUrn: string;

  const dispatch = createEventDispatcher();
</script>

<style>
  .peer {
    display: flex;
    align-items: center;
    padding: 1.375rem 1.5rem;
    width: 100%;
    justify-content: space-between;
  }
  .left {
    max-width: 22em;
    flex-direction: column;
  }
</style>

<div class="peer" data-cy={`peer-${peer.identity.metadata.handle}`}>
  <div class="left">
    <div style="display: flex; gap: 0.5rem;">
      <UserIdentity
        urn={peer.identity.urn}
        handle={peer.identity.metadata.handle} />
      <UserBadge user={peer} />
    </div>
    <CopyableIdentifier
      value={peer.peerId}
      kind="peerId"
      style="margin-top: 0.5rem;" />
  </div>
  {#if peer.type !== project.PeerType.Local}
    {#if peer.role === project.PeerRole.Delegate}
      <Tooltip position="top" value="Can't untrack the delegate's remote">
        <TrackToggle disabled tracking />
      </Tooltip>
    {:else}
      <TrackToggle
        tracking
        on:untrack={() => {
          dispatch("untrack", { projectUrn, peerId: peer.peerId });
        }} />
    {/if}
  {/if}
</div>
