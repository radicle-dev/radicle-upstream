<script>
  import { Flex } from "../../DesignSystem/Primitive";
  import TrackToggle from "./TrackToggle.svelte";

  export let remote = null;
  export let projectName = null;

  const firstSix = remote.peerID.substring(0, 7);
  const lastSix = remote.peerID.substring(
    remote.peerID.length - 7,
    remote.peerID.length
  );
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
    <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
      {firstSix}...{lastSix} / {projectName}
    </p>
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    <TrackToggle />
  </div>
</Flex>
