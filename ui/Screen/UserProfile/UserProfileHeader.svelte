<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Avatar as AvatarT } from "ui/src/proxy/identity";

  import { Avatar, PeerId } from "ui/DesignSystem";

  export let name: string;
  export let peerId: string;

  export let avatarFallback: AvatarT;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
</style>

<Avatar
  style="margin-right: 32px"
  size="huge"
  variant="circle"
  {avatarFallback} />

<div class="metadata">
  <h1 data-cy="entity-name" class="typo-overflow-ellipsis" title={name}>
    {name}
  </h1>
  <PeerId truncate {peerId} style="margin-top: 0.5rem;" />
</div>
