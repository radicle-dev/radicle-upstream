<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Urn } from "../../src/urn";
  import type { User } from "ui/src/project";

  import { FollowToggle, StyledCopyable } from "ui/DesignSystem";

  export let peer: User;
  export let projectUrn: Urn;

  const dispatch = createEventDispatcher();
</script>

<style>
  .peer-request {
    display: flex;
    padding: 1.375rem 1.5rem;
    width: 100%;
    justify-content: space-between;
  }
  .left {
    max-width: 22em;
  }
</style>

<div class="peer-request" data-cy="peer-request">
  <div class="left" style="max-width: 22em">
    <StyledCopyable
      style="margin: 0.5rem 0 0 -0.25rem;"
      truncate
      expandable={false}
      value={peer.peerId} />
  </div>

  <FollowToggle
    following
    on:unfollow={() => {
      dispatch("cancel", { projectUrn, peerId: peer.peerId });
    }} />
</div>
