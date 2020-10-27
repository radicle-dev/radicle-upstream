<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "../../src/urn";
  import type { User } from "../../src/project";

  import { Flex } from "../../DesignSystem/Primitive";
  import { FollowToggle } from "../../DesignSystem/Component";

  export let peer: User;
  export let projectName: string;
  export let projectUrn: Urn;

  const dispatch = createEventDispatcher();

  const [head, tail] = peer.peerId.split(/(.{6}).*(.{6})/).filter(Boolean);
</script>

<Flex style="flex: 1; padding: 1.375rem 1.5rem;">
  <div slot="left" style="max-width: 22em">
    <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
      {head}…{tail} / {projectName}
    </p>
  </div>
  <div slot="right" style="display: flex; align-items: center;">
    <FollowToggle
      expanded
      following
      on:unfollow={() => {
        dispatch('cancel', { projectUrn, peerId: peer.peerId });
      }} />
  </div>
</Flex>
