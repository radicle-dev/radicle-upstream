<script>
  import { BadgeType } from "../../src/badge.ts";
  import { PeerType, removeRemote, Role } from "../../src/project";
  import { CSSPosition } from "../../src/style";
  
  import { Avatar, Flex } from "../../DesignSystem/Primitive";
  
  import Badge from "./Badge.svelte";
  import Tooltip from "./Tooltip.svelte";
  import TrackToggle from "./TrackToggle.svelte";


  export let peer = null;
  export let projectId = null;
  export let projectName = null;

  const unfollowRemote = () => {
    removeRemote(projectId, peer.peerId);
  };
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
        <TrackToggle disabled tracking expanded />
      </Tooltip>
    {:else if peer.role === Role.Maintainer}
      <Tooltip
        position={CSSPosition.Top}
        value="Can't unfollow the maintainer's remote">
        <TrackToggle disabled tracking expanded />
      </Tooltip>
    {:else}
      <Tooltip>
        <TrackToggle tracking expanded on:untrack={unfollowRemote} />
      </Tooltip>
    {/if}
  </div>
</Flex>
