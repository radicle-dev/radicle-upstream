<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { BadgeType } from "../../../src/badge";
  import { Role } from "../../../src/project";
  import type { User } from "../../../src/project";
  import { CSSPosition } from "../../../src/style";

  import Avatar from "../../Primitive/Avatar.svelte";
  import IconArrowBoxUpRight from "../../Primitive/Icon/ArrowBoxUpRight.svelte";

  import Badge from "../Badge.svelte";
  import Tooltip from "../Tooltip.svelte";

  export let peer: User;
  export let showProfile: boolean = false;

  const dispatch = createEventDispatcher();
</script>

<style>
  .open-profile {
    cursor: pointer;
    display: flex;
    justify-content: center;
    margin-left: 0.5rem;
  }
</style>

<div data-peer-handle={peer.identity.metadata.handle} style="display: flex;">
  <Avatar
    avatarFallback={peer.identity.avatarFallback}
    style="display: flex; justify-content: flex-start; margin-right:
  8px;"
    size="small"
    variant="circle" />
  <p class="typo-text-bold typo-overflow-ellipsis">
    {peer.identity.metadata.handle || peer.identity.shareableEntityIdentifier}
  </p>
  <p>
    {#if peer.role === Role.Maintainer}
      <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
    {/if}
  </p>
</div>

{#if showProfile}
  <Tooltip value="Go to profile" position={CSSPosition.Top}>
    <div
      data-cy={peer.identity.metadata.handle}
      class="open-profile"
      on:click|stopPropagation={() => dispatch('open')}>
      <IconArrowBoxUpRight />
    </div>
  </Tooltip>
{/if}
