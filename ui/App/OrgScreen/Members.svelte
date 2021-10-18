<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as org from "ui/src/org";
  import type * as project from "ui/src/project";
  import * as userProfile from "ui/src/userProfile";

  import { Avatar, CopyableIdentifier, List } from "ui/DesignSystem";

  export let members: org.Member[];

  function openUserProfile({ detail: member }: { detail: project.User }) {
    if (member.identity) {
      userProfile.openUserProfile(member.identity.urn);
    }
  }
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
  <List items={members} let:item={member} on:select={openUserProfile}>
    <div class="list-item">
      <div>
        {#if member.identity}
          <div style="display: flex">
            <div style="display: flex; margin-right: 1rem;">
              <Avatar
                style="margin-right: 0.625rem;"
                size="small"
                kind={{
                  type: "userEmoji",
                  uniqueIdentifier: member.identity.urn,
                }} />
              <p class="typo-text">{member.identity.metadata.handle}</p>
            </div>
            <CopyableIdentifier
              value={member.ethereumAddress}
              kind="ethAddress"
              showIcon={false}
              tooltipPosition="right" />
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
      <a class="typo-link url" href={org.etherscanUrl(member.ethereumAddress)}
        >View on Etherscan</a>
    </div>
  </List>
</div>
