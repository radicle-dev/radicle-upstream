<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Avatar from "design-system/Avatar.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let name: string;
  export let peerId: string;
  export let urn: string;
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

<div style="display: flex">
  <Avatar
    style="margin-right: 32px"
    size="huge"
    kind={{ type: "userEmoji", uniqueIdentifier: urn }} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis" title={name}>
      {name}
    </h1>
    <CopyableIdentifier
      value={peerId}
      kind="peerId"
      style="margin-top: 0.5rem;" />
  </div>
</div>
