<script>
  import { Flex } from "../../DesignSystem/Primitive";
  import TrackToggle from "./TrackToggle.svelte";

  export let peer = null;
  export let projectName = null;

  const firstSix = peer.peerId.substring(0, 7);
  const lastSix = peer.peerId.substring(
    peer.peerId.length - 7,
    peer.peerId.length
  );
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
    <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
      {firstSix}...{lastSix} / {projectName}
    </p>
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    <TrackToggle expanded tracking />
  </div>
</Flex>
