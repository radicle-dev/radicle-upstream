<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { PeerRole, PeerType } from "ui/src/project";
  import type { User } from "ui/src/project";

  import { Avatar, Badge } from "ui/DesignSystem";

  export let peer: User;
</script>

<style>
  .peer {
    display: flex;
    justify-content: flex-start;
  }

  p.name,
  p.badge {
    margin-left: 0.5rem;
  }
</style>

<div class="peer" data-peer-handle={peer.identity.metadata.handle}>
  <Avatar
    kind={{ type: "userEmoji", uniqueIdentifier: peer.identity.urn }}
    size="small" />
  <p class="name typo-text-bold typo-overflow-ellipsis">
    {peer.identity.metadata.handle}
  </p>
  {#if peer.role === PeerRole.Maintainer}
    <p class="badge">
      <Badge variant="maintainer" />
    </p>
  {:else if peer.type === PeerType.Local}
    <p class="badge">
      <Badge variant="you" />
    </p>
  {/if}
</div>
