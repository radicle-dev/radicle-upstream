<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { BadgeType } from "../../src/badge";
  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { Role } from "../../src/project";
  import type { User } from "../../src/project";
  import { CSSPosition } from "../../src/style";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { Badge, Overlay, Tooltip } from "../../DesignSystem/Component";

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
  const open = (peer: User) => {
    hide();
    dispatch("open", peer);
  };
  const select = (peer: User) => {
    hide();
    dispatch("select", peer);
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

  .peer {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    height: 2.5rem;
    user-select: none;
    align-items: center;
    justify-content: space-between;
    background-color: var(--color-background);
  }

  .peer:first-child {
    border-radius: 0.1875rem 0.1875rem 0 0;
  }
  .peer:last-child {
    border-radius: 0 0 0.1875rem 0.1875rem;
  }

  .peer.selected {
    background-color: var(--color-foreground-level-2);
  }

  .peer:hover {
    cursor: pointer;
    background-color: var(--color-foreground-level-2);
  }

  .open-profile {
    display: flex;
    justify-content: center;
    cursor: pointer;
    margin-left: 0.5rem;
  }

  .remotes {
    justify-content: flex-start;
  }

  .remotes p {
    white-space: nowrap;
    margin-right: 0.5rem;
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
        <slot name="badge" peer={selected} />
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
        <div
          class="peer"
          class:selected={peer.identity.peerId == selected.identity.peerId}
          data-peer-handle={peer.identity.metadata.handle}
          on:click={() => select(peer)}>
          <div style="display: flex;">
            <Avatar
              avatarFallback={peer.identity.avatarFallback}
              style="display: flex; justify-content: flex-start; margin-right:
            8px;"
              size="small"
              variant="circle" />
            <p class="typo-text-bold typo-overflow-ellipsis">
              {peer.identity.metadata.handle || peer.identity.shareableEntityIdentifier}
            </p>
            <p>
              {#if peer.role === Role.Maintainer}
                <Badge
                  style="margin-left: 0.5rem"
                  variant={BadgeType.Maintainer} />
              {/if}
            </p>
          </div>
          <Tooltip value="Go to profile" position={CSSPosition.Top}>
            <div
              data-cy={peer.identity.metadata.handle}
              class="open-profile"
              on:click={() => {
                open(peer);
              }}>
              <Icon.ArrowBoxUpRight />
            </div>
          </Tooltip>
        </div>
      {/each}
      <div
        class="peer remotes"
        data-cy="manage-remotes"
        on:click={() => modal.toggle(path.managePeers())}>
        <Icon.Pen style="margin-right: .5rem;" />
        <p>Manage remotes</p>
      </div>
    </div>
  </div>
</Overlay>
