<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "../../src/urn";
  import type { User } from "../../src/project";

  import { BadgeType } from "../../src/badge";
  import { CSSPosition } from "../../src/style";
  import { PeerType, Role } from "../../src/project";

  import { Avatar, Flex } from "../../DesignSystem/Primitive";
  import { Badge, FollowToggle, Tooltip } from "../../DesignSystem/Component";

  export let peer: User;
  export let projectUrn: Urn;
  export let projectName: string;

  const dispatch = createEventDispatcher();
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
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
    <p
      class="typo-text typo-overflow-ellipsis"
      style="color: var(--color-foreground-level-5); padding-top: 0.5rem;">
      {peer.identity.peerId}
    </p>
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    {#if peer.type === PeerType.Local}
      <Tooltip
        position={CSSPosition.Top}
        value="Can't unfollow your own remote">
        <FollowToggle disabled following expanded />
      </Tooltip>
    {:else if peer.role === Role.Maintainer}
      <Tooltip
        position={CSSPosition.Top}
        value="Can't unfollow the maintainer's remote">
        <FollowToggle disabled following expanded />
      </Tooltip>
    {:else}
      <Tooltip>
        <FollowToggle
          following
          expanded
          on:untrack={() => {
            dispatch('unfollow', { projectUrn, peerId: peer.peerId });
          }} />
      </Tooltip>
    {/if}
  </div>
</Flex>
