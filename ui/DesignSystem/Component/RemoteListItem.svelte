<script>
  import { Avatar, Flex } from "../../DesignSystem/Primitive";
  import TrackToggle from "./TrackToggle.svelte";
  export let remote = null;

  const firstSix = remote.peerID.substring(0, 7);
  const lastSix = remote.peerID.substring(
    remote.peerID.length - 7,
    remote.peerID.length
  );
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
    {#if remote.handle !== null}
      <div style="display: flex;">
        <Avatar
          size="small"
          style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
          variant="circle" />
        <p
          class="typo-text-bold"
          style="color: var(--color-foreground-level-6);">
          {remote.handle} / sup
        </p>
      </div>
      <p
        class="typo-text typo-overflow-ellipsis"
        style="color: var(--color-foreground-level-5); padding-top: 0.5rem;">
        {remote.peerID}
      </p>
    {:else if remote.synced !== true}
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
        {firstSix}...{lastSix} / sup
      </p>
    {/if}
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    <TrackToggle />
  </div>
</Flex>
