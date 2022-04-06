<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { User } from "ui/src/project";

  import { createEventDispatcher } from "svelte";

  import TrackToggle from "design-system/TrackToggle.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let peer: User;
  export let projectUrn: string;

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
    <CopyableIdentifier
      value={peer.peerId}
      kind="peerId"
      style="margin-top: 0.5rem" />
  </div>

  <TrackToggle
    tracking
    on:untrack={() => {
      dispatch("cancel", { projectUrn, peerId: peer.peerId });
    }} />
</div>
