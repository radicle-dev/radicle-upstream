<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { BadgeType } from "../../src/badge";
  import { isExperimental } from "../../src/ipc";
  import { Role } from "../../src/project";
  import type { User } from "../../src/project";

  import { Avatar, Icon } from "../Primitive";
  import { Badge, Overlay } from "../Component";

  import Entry from "./PeerSelector/Entry.svelte";
  import Peer from "./PeerSelector/Peer.svelte";

  export let expanded: boolean = false;
  export let peers: User[];
  export let selected: User;

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
  const showProfile = isExperimental();
  const tooltip = (peer: User): string | null => {
    if (peer.role === Role.Tracker) {
      return "Remote has no changes";
    }
    return null;
  };
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

  .selector-avatar {
    display: flex;
    justify-content: space-between;
    width: 100%;
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
  }
</style>

<Overlay
  {expanded}
  on:hide={hide}
  style="margin-right: 1rem; position: relative; user-select: none;">
  <div
    class="peer-selector"
    data-cy="peer-selector"
    on:click|stopPropagation={show}
    hidden={expanded}>
    <div class="selector-avatar typo-overflow-ellipsis">
      <Avatar
        avatarFallback={selected.identity.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p
        class="typo-text-bold typo-overflow-ellipsis"
        style="max-width: 7.5rem;"
        title={selected.identity.metadata.handle || selected.identity.shareableEntityIdentifier}>
        {selected.identity.metadata.handle || selected.identity.shareableEntityIdentifier}
      </p>
      <p>
        {#if selected.role === Role.Maintainer}
          <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
        {/if}
      </p>
    </div>
    <div class="selector-expand">
      <Icon.ChevronUpDown
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="peer-dropdown-container">
    <div class="peer-dropdown" hidden={!expanded}>
      {#each peers as peer}
        <Entry
          active={peer.role !== Role.Tracker}
          on:click={() => onSelect(peer)}
          selected={peer.identity.peerId == selected.identity.peerId}
          tooltip={tooltip(peer)}>
          <Peer {peer} {showProfile} on:open={() => onOpen(peer)} />
        </Entry>
      {/each}

      <Entry
        dataCy="manage-remotes"
        on:click={onModal}
        style="justify-content: flex-start;">
        <Icon.Pen style="margin-right: .5rem;" />
        <p>Manage remotes</p>
      </Entry>
    </div>
  </div>
</Overlay>
