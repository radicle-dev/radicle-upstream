<script lang="ts">
  import { createEventDispatcher, getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as badge from "../../src/badge";
  import * as identity from "../../src/identity";
  import * as path from "../../src/path";
  import * as style from "../../src/style";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { Badge, Overlay, Tooltip } from "../../DesignSystem/Component";

  export let currentPeerId: string | undefined;
  export let availablePeers: identity.Identity[];
  export let maintainers: string[] | undefined = undefined;

  let expanded = false,
    currentPeer: identity.Identity;

  const session = getContext("session");

  const showDropdown = () => {
    expanded = true;
  };

  const hideDropdown = () => {
    expanded = false;
  };

  const handleOpenProfile = (urn: string) => {
    if (urn === session.identity.urn) {
      push(path.profileProjects());
    } else {
      push(path.userProfileProjects(urn));
    }
  };

  const dispatch = createEventDispatcher();
  const selectPeer = (peerId: string) => {
    hideDropdown();
    dispatch("select", { peerId });
  };

  $: currentPeer =
    availablePeers.find(peer => peer.id === currentPeerId) || availablePeers[0];
</script>

<style>
  .peer-selector {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    padding: 0.5rem;
    display: flex;
    cursor: pointer;
    justify-content: space-between;
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
    align-self: flex-end;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: 0;
    cursor: pointer;
  }

  .peer-dropdown {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
  }

  .peer {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    user-select: none;
    align-items: center;
    justify-content: space-between;
  }

  .open-profile {
    display: flex;
    justify-content: center;
    cursor: pointer;
  }
</style>

<Overlay {expanded} on:hide={hideDropdown} style="position: relative;">
  {#if currentPeer}
    <div
      class="peer-selector"
      data-cy="peer-selector"
      on:click|stopPropagation={showDropdown}
      hidden={expanded}>
      <div class="selector-avatar typo-overflow-ellipsis">
        <Avatar
          avatarFallback={currentPeer.avatarFallback}
          size="small"
          style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
          variant="circle" />
        <p class="typo-text-bold typo-overflow-ellipsis">
          {currentPeer.metadata.handle || currentPeer.shareableEntityIdentifier}
        </p>
        <p>
          {#if maintainers && maintainers.includes(currentPeer.urn)}
            <Badge
              style="margin-left: 0.5rem"
              variant={badge.BadgeType.Maintainer} />
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
        {#each availablePeers as peer}
          <div
            class="peer"
            class:selected={peer.peerId == currentPeer.peerId}
            data-peer-handle={peer.metadata.handle}>
            <div
              style="display: flex;"
              on:click={() => selectPeer(peer.peerId)}>
              <Avatar
                avatarFallback={peer.avatarFallback}
                style="display: flex; justify-content: flex-start; margin-right:
            8px;"
                size="small"
                variant="circle" />
              <p class="typo-text-bold typo-overflow-ellipsis">
                {peer.metadata.handle || peer.shareableEntityIdentifier}
              </p>
              <p>
                {#if maintainers && maintainers.includes(peer.urn)}
                  <Badge
                    style="margin-left: 0.5rem"
                    variant={badge.BadgeType.Maintainer} />
                {/if}
              </p>
            </div>
            <Tooltip value="Go to profile" position={style.CSSPosition.Top}>
              <div
                data-cy={peer.metadata.handle}
                class="open-profile"
                on:click={() => {
                  handleOpenProfile(peer.urn);
                }}>
                <Icon.ArrowBoxUpRight />
              </div>
            </Tooltip>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</Overlay>
