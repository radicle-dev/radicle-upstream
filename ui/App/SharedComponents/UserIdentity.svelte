<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as proxyIdentity from "proxy-client/identity";

  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";
  import * as session from "ui/src/session";

  import Avatar from "design-system/Avatar.svelte";
  import Hovercard from "design-system/Hovercard.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let urn: string;
  export let handle: string | undefined = undefined;
  export let disableHovercard: boolean = false;
  export let modalStyle: string = "top: -2rem; left: -17rem;";
  export let triggerStyle: string | undefined = undefined;
  export let boldHandle: boolean = false;

  let user: proxyIdentity.RemoteIdentity | undefined = undefined;

  async function fetchUser(urn: string): Promise<void> {
    // Load data only the first time a user hovers a UserIdentity component and
    // make sure we show the correct data when the URN changes due to
    // reactivity.
    if (user?.urn === urn) {
      return;
    } else {
      user = undefined;
    }

    try {
      user = await proxy.client.personGet(urn);
    } catch (err: unknown) {
      notification.showException(
        new error.Error({
          message: "Failed to fetch user data",
          source: err,
        })
      );
    }
  }

  function goToProfile(urn: string): void {
    modal.hide();
    if (urn === session.unsealed().identity.urn) {
      router.push({ type: "profile" });
    } else {
      router.push({
        type: "userProfile",
        params: { urn },
      });
    }
  }
</script>

<style>
  .card {
    width: 16rem;
    color: var(--color-foreground);
  }

  button {
    border-top: 1px solid var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    height: 3.5rem;
    width: 100%;
    cursor: pointer;
  }

  .top {
    margin: 1.5rem auto;
  }

  .metadata {
    gap: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .user {
    display: flex;
    cursor: pointer;
  }

  .name {
    margin-left: 0.5rem;
  }

  .handle-wrapper {
    max-width: 100%;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 1rem;
  }
</style>

<Hovercard
  disabled={disableHovercard}
  {modalStyle}
  onHover={() => fetchUser(urn)}>
  <svelte:fragment slot="trigger">
    <div class="user" data-peer-handle={handle} style={triggerStyle}>
      <Avatar
        kind={{ type: "userEmoji", uniqueIdentifier: urn }}
        size="small" />
      {#if handle}
        <div
          class:typo-text-bold={boldHandle}
          class="name typo-overflow-ellipsis">
          {handle}
        </div>
      {/if}
    </div>
  </svelte:fragment>

  <div class="card" slot="card">
    <div class="top">
      <Avatar
        style="margin-bottom: 1rem;"
        size="large"
        kind={{
          type: "userEmoji",
          uniqueIdentifier: urn,
        }} />

      <div class="metadata">
        {#if user}
          <div class="handle-wrapper">
            <h2 class="typo-overflow-ellipsis">
              {handle || user.metadata.handle}
            </h2>
          </div>
          {#if user.metadata.ethereum}
            <CopyableIdentifier
              kind="ethAddress"
              value={user.metadata.ethereum.address} />
          {/if}
          {#each user.peerIds as peerId}
            <CopyableIdentifier kind="peerId" value={peerId} />
          {/each}
        {/if}
      </div>
    </div>

    <button
      data-cy="view-profile-button"
      class="typo-text-bold"
      on:click={() => {
        goToProfile(urn);
      }}>View profile</button>
  </div>
</Hovercard>
