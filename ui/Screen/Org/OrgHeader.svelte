<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import { Avatar, Icon, Identifier } from "ui/DesignSystem";

  import * as format from "ui/src/format";

  export let orgAddress: string;
  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
  .name {
    margin-bottom: 0.5rem;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
</style>

<div style="display: flex">
  <Avatar
    style="margin-right: 2rem;"
    size="huge"
    variant="square"
    avatarFallback={radicleAvatar.generate(
      orgAddress,
      radicleAvatar.Usage.Any
    )} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
      {format.shortEthAddress(orgAddress)}
    </h1>
    <div class="row">
      {#if threshold}
        <Icon.Gnosis />
      {:else}
        <Icon.Ethereum />
      {/if}
      <Identifier
        value={ownerAddress}
        kind="ethAddress"
        name="org owner address"
        showIcon={false} />
    </div>
    {#if threshold}
      <div class="row">
        <Icon.Orgs />
        {threshold}
        {threshold === 1 ? "signature" : "signatures"} required for quorum
      </div>
    {/if}
  </div>
</div>
