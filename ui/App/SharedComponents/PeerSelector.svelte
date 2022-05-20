<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { PeerRole } from "ui/src/project";
  import type { User } from "ui/src/project";

  import ChevronUpDownIcon from "design-system/icons/ChevronUpDown.svelte";

  import Overlay from "design-system/Overlay.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";
  import UserBadge from "ui/App/SharedComponents/UserBadge.svelte";

  export let expanded: boolean = false;
  // If `true`,  this component is used in a stand-alone context. This means it
  // has all corners rounded and it acts like a push button.
  // If `false`, this component is used together with the "Edit Remotes" button
  // on the right-hand side and doesn't animate when clicked.
  export let standalone: boolean = false;
  export let peers: User[];
  export let selected: User;

  const orderPeers = (peers: User[]): User[] => {
    return [selected].concat(
      peers.filter(peer => peer.peerId !== selected.peerId)
    );
  };

  const hide = (): void => {
    expanded = false;
  };
  const show = (): void => {
    expanded = true;
  };

  const dispatch = createEventDispatcher();
  const onSelect = (peer: User): void => {
    if (peer.role === PeerRole.Tracker) {
      return;
    }
    hide();
    dispatch("select", peer);
  };

  let dropdownWidth: number;
</script>

<style>
  .peer-selector {
    display: flex;
    border: 1px solid var(--color-foreground-level-3);
    border-right: none;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
    padding: 0 0.5rem;
    align-items: center;
    height: 2.5rem;
    cursor: pointer;
    justify-content: space-between;
    background-color: var(--color-foreground-level-1);
    user-select: none;
    gap: 0.5rem;
  }

  .peer-selector:hover {
    background-color: var(--color-foreground-level-2);
  }

  .peer-selector[hidden] {
    visibility: hidden;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: -1px;
    user-select: none;
  }

  .peer-dropdown {
    border: 1px solid transparent;
    border-right: none;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
    overflow: hidden;
  }

  .rounded {
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
  }

  .entry {
    align-items: center;
    background-color: var(--color-background);
    color: var(--color-foreground-level-3);
    gap: 0.5rem;
    cursor: not-allowed;
    display: flex;
    height: 2.5rem;
    padding: 0 0.5em;
  }

  .entry.enabled {
    color: var(--color-foreground-level-6);
  }

  .entry.enabled:hover {
    background-color: var(--color-foreground-level-2);
    cursor: pointer;
  }

  .entry.selected {
    background-color: var(--color-foreground-level-2);
  }

  .rounded-bottom-right:last-child {
    border-bottom-right-radius: 0.5rem;
  }
</style>

<Overlay
  {expanded}
  on:hide={hide}
  style="position: relative; user-select: none;">
  <div
    role="button"
    aria-label="select peer"
    bind:clientWidth={dropdownWidth}
    class="peer-selector"
    class:button-transition={standalone}
    class:rounded={standalone}
    data-cy="peer-selector"
    hidden={expanded}
    on:click|stopPropagation={show}>
    <UserIdentity
      triggerStyle="max-width: 24rem;"
      boldHandle={true}
      urn={selected.identity.urn}
      handle={selected.identity.metadata.handle} />
    <UserBadge user={selected} />
    <ChevronUpDownIcon style="fill: var(--color-foreground-level-4);" />
  </div>
  <div class="peer-dropdown-container" data-cy="peer-dropdown-container">
    <div
      class="peer-dropdown"
      hidden={!expanded}
      style={`min-width: ${dropdownWidth ? dropdownWidth : "0"}px;`}
      class:rounded={standalone}
      class:rounded-bottom-right={!standalone && peers.length > 1}>
      {#each orderPeers(peers) as peer (peer.peerId)}
        <div
          data-cy="peer-dropdown-entry"
          class="entry rounded-left"
          class:rounded-right={standalone}
          class:rounded-bottom-right={!standalone && peers.length > 1}
          class:enabled={peer.role !== PeerRole.Tracker}
          class:selected={peer.identity.peerId === selected.identity.peerId}
          on:click|stopPropagation={() => onSelect(peer)}>
          {#if peer.role === PeerRole.Tracker}
            <Tooltip position="top" value="Remote has no changes">
              <UserIdentity
                disableHovercard={true}
                boldHandle={true}
                urn={peer.identity.urn}
                handle={peer.identity.metadata.handle} />
              <UserBadge user={peer} />
            </Tooltip>
          {:else}
            <UserIdentity
              disableHovercard={true}
              boldHandle={true}
              urn={peer.identity.urn}
              handle={peer.identity.metadata.handle} />
            <UserBadge user={peer} />
          {/if}
        </div>
      {/each}
    </div>
  </div>
</Overlay>
