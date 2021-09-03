<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "ui/src/urn";
  import type { User } from "ui/src/project";

  import { PeerType, PeerRole } from "ui/src/project";

  import {
    Avatar,
    Badge,
    FollowToggle,
    Identifier,
    Tooltip,
  } from "ui/DesignSystem";

  export let peer: User;
  export let projectUrn: Urn;

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
  .user-profile {
    display: inline-flex;
    cursor: pointer;
  }
</style>

<div class="peer" data-cy={`peer-${peer.identity.metadata.handle}`}>
  <div class="left">
    <div
      class="user-profile"
      on:click={() => {
        dispatch("userProfileClick", { urn: peer.identity.urn });
      }}>
      <Avatar
        avatarFallback={peer.identity.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
        {peer.identity.metadata.handle}
      </p>
      {#if peer.role === PeerRole.Maintainer}
        <Badge style="margin-left: 0.5rem" variant="maintainer" />
      {:else if peer.type === PeerType.Local}
        <Badge style="margin-left: 0.5rem" variant="you" />
      {/if}
    </div>
    <Identifier
      value={peer.peerId}
      kind="deviceId"
      style="margin-top: 0.5rem;" />
  </div>
  {#if peer.type !== PeerType.Local}
    {#if peer.role === PeerRole.Maintainer}
      <Tooltip position="top" value="Can't unfollow the maintainer's remote">
        <FollowToggle disabled following />
      </Tooltip>
    {:else}
      <FollowToggle
        following
        on:unfollow={() => {
          dispatch("unfollow", { projectUrn, peerId: peer.peerId });
        }} />
    {/if}
  {/if}
</div>
