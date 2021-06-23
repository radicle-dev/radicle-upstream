<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Avatar, List, StyledCopyable } from "ui/DesignSystem";
  import type * as org from "ui/src/org";

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
    height: 70px;
    justify-content: space-between;
    padding: 1rem;
    align-items: center;
    min-width: 0;
  }

  .member-details {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
</style>

<div class="container">
  <List items={members} let:item={member} styleHoverState={false}>
    <div class="list-item">
      {#if member.identity}
        <div style="display: flex">
          <Avatar
            style="margin-right: 10px"
            size="medium"
            variant="circle"
            avatarFallback={member.identity.avatarFallback} />
          <div
            class="member-details"
            data-cy="entity-name"
            title={member.identity.metadata.handle}>
            {member.identity.metadata.handle}
          </div>
        </div>
      {:else}
        <div
          class="member-details"
          data-cy="entity-name"
          title="Unknown identity">
          {member.ethereumAddress}
        </div>
      {/if}
      <StyledCopyable truncate value={member.ethereumAddress} />
    </div>
  </List>
</div>
