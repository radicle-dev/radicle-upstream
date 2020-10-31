<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "../../src/urn";
  import type { User } from "../../src/project";

  import { BadgeType } from "../../src/badge";
  import { CSSPosition } from "../../src/style";
  import { PeerType, Role } from "../../src/project";

  import { Avatar } from "../../DesignSystem/Primitive";
  import {
    Badge,
    FollowToggle,
    StyledCopyable,
    Tooltip,
  } from "../../DesignSystem/Component";

  export let peer: User;
  export let projectUrn: Urn;
  export let projectName: string;

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

<div class="peer" data-cy="peer">
  <div class="left">
    <div style="display: flex;">
      <Avatar
        avatarFallback={peer.identity.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
        {peer.identity.metadata.handle} / {projectName}
      </p>
      {#if peer.role === Role.Maintainer}
        <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
      {/if}
    </div>
    <StyledCopyable
      style="margin: 0.5rem 0 0 -0.25rem;"
      truncate
      expandable={false}
      value={peer.peerId} />
  </div>
  {#if peer.type !== PeerType.Local}
    {#if peer.role === Role.Maintainer}
      <Tooltip
        position={CSSPosition.Top}
        value="Can't unfollow the maintainer's remote">
        <FollowToggle disabled following expanded />
      </Tooltip>
    {:else}
      <FollowToggle
        following
        expanded
        on:unfollow={() => {
          dispatch('unfollow', { projectUrn, peerId: peer.peerId });
        }} />
    {/if}
  {/if}
</div>