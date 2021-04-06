<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { isExperimental } from "../../src/config";
  import { Role } from "../../src/project";
  import type { User } from "../../src/project";
  import { CSSPosition } from "../../src/style";

  import { Icon } from "../Primitive";
  import { Overlay, Tooltip } from "../Component";

  import Entry from "./PeerSelector/Entry.svelte";
  import Peer from "./PeerSelector/Peer.svelte";

  export let expanded: boolean = false;
  export let peers: User[];
  export let selected: User;

  const orderPeers = (peers: User[]): User[] => {
    return [selected].concat(
      peers.filter(peer => peer.peerId !== selected.peerId)
    );
  };

  const hide = () => {
    expanded = false;
  };
  const show = () => {
    expanded = true;
  };

  const dispatch = createEventDispatcher();
  const onModal = () => {
    hide();
    dispatch("modal");
  };
  const onOpen = (peer: User) => {
    hide();
    dispatch("open", peer);
  };
  const onSelect = (peer: User) => {
    if (peer.role === Role.Tracker) {
      return;
    }
    hide();
    dispatch("select", peer);
  };
  const showProfile = isExperimental;
</script>

<style>
  .peer-selector {
    display: flex;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    padding: 0 0.5rem;
    align-items: center;
    height: 2.5rem;
    cursor: pointer;
    justify-content: space-between;
    background-color: var(--color-foreground-level-1);
    user-select: none;
  }

  .peer-selector:hover {
    color: var(--color-foreground);
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);
  }

  .peer-selector[hidden] {
    visibility: hidden;
  }

  .selector-expand {
    margin-left: 0.5rem;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: -1px;
    user-select: none;
  }

  .peer-dropdown {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
    overflow: hidden;
  }

  .action {
    height: 1.5rem;
    margin-left: 0.5rem;
    width: 1.5rem;
  }

  .open-profile {
    cursor: pointer;
    display: flex;
    justify-content: center;
  }

  p.remotes {
    white-space: nowrap;
  }
</style>

<Overlay
  {expanded}
  on:hide={hide}
  style="margin-right: 1rem; position: relative; user-select: none;">
  <div
    class="peer-selector typo-overflow-ellipsis"
    data-cy="peer-selector"
    hidden={expanded}
    on:click|stopPropagation={show}>
    <Peer peer={selected} />
    <div class="selector-expand">
      <Icon.ChevronUpDown
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="peer-dropdown-container" data-cy="peer-dropdown-container">
    <div class="peer-dropdown" hidden={!expanded}>
      {#each orderPeers(peers) as peer (peer.peerId)}
        <Entry
          disabled={peer.role === Role.Tracker}
          on:click={() => onSelect(peer)}
          selected={peer.identity.peerId === selected.identity.peerId}>
          {#if peer.role === Role.Tracker}
            <Tooltip position={CSSPosition.Left} value="Remote has no changes">
              <Peer {peer} />
            </Tooltip>
          {:else}
            <Peer {peer} />
          {/if}

          <div class="action">
            {#if showProfile}
              <Tooltip value="Go to profile" position={CSSPosition.Top}>
                <div
                  class="open-profile"
                  data-cy={peer.identity.metadata.handle}
                  on:click|stopPropagation={() => onOpen(peer)}>
                  <Icon.ArrowBoxUpRight />
                </div>
              </Tooltip>
            {/if}
          </div>
        </Entry>
      {/each}

      <Entry
        dataCy="manage-remotes"
        on:click={onModal}
        style="justify-content: flex-start;">
        <Icon.Pen style="margin-right: .5rem;" />
        <p class="remotes">Manage remotes</p>
      </Entry>
    </div>
  </div>
</Overlay>
