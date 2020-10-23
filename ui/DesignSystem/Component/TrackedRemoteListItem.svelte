<script>
  import { Avatar, Flex } from "../../DesignSystem/Primitive";
  import TrackToggle from "./TrackToggle.svelte";
  import Badge from "./Badge.svelte";
  import { BadgeType } from "../../src/badge.ts";

  export let remote = null;
  export let projectName = null;
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
    <div style="display: flex;">
      <Avatar
        avatarFallback={remote.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
        {remote.handle} / {projectName}
      </p>
      {#if remote.maintainer}
        <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
      {/if}
    </div>
    <p
      class="typo-text typo-overflow-ellipsis"
      style="color: var(--color-foreground-level-5); padding-top: 0.5rem;">
      {remote.peerID}
    </p>
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    <TrackToggle disabled tracking expanded />
  </div>
</Flex>
