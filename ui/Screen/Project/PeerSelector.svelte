<script>
  import { createEventDispatcher, getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { BadgeType } from "../../src/badge.ts";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { Badge, Overlay, Tooltip } from "../../DesignSystem/Component";

  export let currentPeerId = null;
  export let expanded = false;
  export let revisions = null;

  let currentSelectedPeer;

  const session = getContext("session");
  const { metadata } = getContext("project");

  $: if (currentPeerId) {
    currentSelectedPeer = revisions.find(rev => {
      return rev.identity.peerId === currentPeerId;
    });
  } else {
    // The API returns a revision list where the first entry is the default
    // peer.
    currentSelectedPeer = revisions[0];
    currentPeerId = currentSelectedPeer.identity.peerId;
  }

  const showDropdown = () => {
    expanded = true;
  };

  const hideDropdown = () => {
    expanded = false;
  };

  const handleOpenProfile = urn => {
    if (urn === session.identity.urn) {
      push(path.profileProjects());
    } else {
      push(path.userProfileProjects(urn));
    }
  };

  const dispatch = createEventDispatcher();
  const selectPeer = peerId => {
    hideDropdown();
    currentPeerId = peerId;
    dispatch("select", { peerId });
  };
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
    margin-left: 0.5rem;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: 0;
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
  on:hide={hideDropdown}
  style="margin-right: 1rem; position: relative; user-select: none;">
  <div
    class="peer-selector"
    data-cy="peer-selector"
    on:click|stopPropagation={showDropdown}
    hidden={expanded}>
    <div class="selector-avatar typo-overflow-ellipsis">
      <Avatar
        avatarFallback={currentSelectedPeer.identity.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold typo-overflow-ellipsis">
        {currentSelectedPeer.identity.metadata.handle || currentSelectedPeer.identity.shareableEntityIdentifier}
      </p>
      <p>
        {#if metadata.maintainers.includes(currentSelectedPeer.identity.urn)}
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
      {#each revisions as repo}
        <div
          class="peer"
          on:click={() => selectPeer(repo.identity.peerId)}
          class:selected={repo.identity.peerId == currentSelectedPeer.identity.peerId}
          data-peer-handle={repo.identity.metadata.handle}>
          <div style="display: flex;">
            <Avatar
              avatarFallback={repo.identity.avatarFallback}
              style="display: flex; justify-content: flex-start; margin-right:
            8px;"
              size="small"
              variant="circle" />
            <p class="typo-text-bold typo-overflow-ellipsis">
              {repo.identity.metadata.handle || repo.identity.shareableEntityIdentifier}
            </p>
            <p>
              {#if metadata.maintainers.includes(repo.identity.urn)}
                <Badge
                  style="margin-left: 0.5rem"
                  variant={BadgeType.Maintainer} />
              {/if}
            </p>
          </div>
          <Tooltip value="Go to profile" position="top">
            <div
              data-cy={repo.identity.metadata.handle}
              class="open-profile"
              on:click={() => {
                handleOpenProfile(repo.identity.urn);
              }}>
              <Icon.ArrowBoxUpRight />
            </div>
          </Tooltip>
        </div>
      {/each}
      <div
        class="peer remotes"
        data-cy="manage-remotes"
        on:click={() => modal.toggle(path.managePeers(), [
            { name: 'metadata', data: metadata },
            { name: 'revisions', data: revisions },
          ])}>
        <Icon.Pen style="margin-right: .5rem;" />
        <p>Manage remotes</p>
      </div>
    </div>
  </div>
</Overlay>
