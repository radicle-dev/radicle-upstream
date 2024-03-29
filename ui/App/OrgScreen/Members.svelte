<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as org from "ui/src/org";

  import Avatar from "design-system/Avatar.svelte";
  import List from "design-system/List.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  export let members: org.Member[];
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .list-item {
    display: flex;
    width: 100%;
    height: 56px;
    justify-content: space-between;
    padding: 1rem;
    align-items: center;
    min-width: 0;
  }
</style>

<div class="container">
  <List items={members} let:item={member} styleHoverState={false}>
    <div class="list-item">
      <div>
        {#if member.identity}
          <div style="display: flex">
            <div style="display: flex; margin-right: 1rem;">
              <UserIdentity
                urn={member.identity.urn}
                handle={member.identity.metadata.handle}
                modalStyle="top: 0.5rem; left: 4rem;" />
            </div>
            <CopyableIdentifier
              value={member.ethereumAddress}
              kind="ethAddress"
              showIcon={false} />
          </div>
        {:else}
          <div style="display: flex; align-items: center;">
            <Avatar
              style="margin-right: 0.625rem;"
              size="small"
              kind={{ type: "unknownUser" }} />
            <CopyableIdentifier
              value={member.ethereumAddress}
              kind="ethAddress"
              showIcon={false} />
          </div>
        {/if}
      </div>
      <a
        on:click|stopPropagation
        class="typo-link url"
        href={org.etherscanUrl(member.ethereumAddress)}>View on Etherscan</a>
    </div>
  </List>
</div>
