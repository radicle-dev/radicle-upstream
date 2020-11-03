<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "../../src/urn";
  import type { User } from "../../src/project";

  import { FollowToggle, StyledCopyable } from "../../DesignSystem/Component";

  export let peer: User;
  export let projectName: string;
  export let projectUrn: Urn;

  const dispatch = createEventDispatcher();
</script>

<style>
  .peer-request {
    display: flex;
    padding: 1.375rem 1.5rem;
    align-items: center;
    width: 100%;
    justify-content: space-between;
  }
  .left {
    max-width: 22em;
  }
</style>

<div class="peer-request" data-cy="peer-request">
  <div class="left" style="max-width: 22em">
    <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
      {projectName}
    </p>
    <StyledCopyable
      style="margin: 0.5rem 0 0 -0.25rem;"
      truncate
      expandable={false}
      value={peer.peerId} />
  </div>

  <FollowToggle
    following
    on:unfollow={() => {
      dispatch('cancel', { projectUrn, peerId: peer.peerId });
    }} />
</div>
