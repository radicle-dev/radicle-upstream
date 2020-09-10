<script>
  import { createEventDispatcher, getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";
  import { BadgeType } from "../../src/badge.ts";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { Tooltip, Badge } from "../../DesignSystem/Component";

  export let currentPeerId = null;
  export let expanded = false;
  export let revisions = null;
  export let maintainers = [];

  let currentSelectedPeer;

  const session = getContext("session");

  $: if (currentPeerId) {
    currentSelectedPeer = revisions.find(rev => {
      return rev.identity.peerId === currentPeerId;
    });
  } else {
    // The API returns a revision list where the first entry is the default
    // peer.
    currentSelectedPeer = revisions[0];
  }

  // Dropdown element. Set by the view.
  let dropdown = null;

  const showDropdown = () => {
    expanded = true;
  };

  const hideDropdown = () => {
    expanded = false;
  };

  const handleClick = ev => {
    // Any click *outside* the dropdown should hide the dropdown.
    if (dropdown !== ev.target && !dropdown.contains(ev.target)) {
      hideDropdown();
    }
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
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
  }

  .peer-dropdown {
    position: relative;
    background: var(--color-background);
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

<svelte:window on:click={handleClick} />
<div
  class="peer-selector"
  data-cy="peer-selector"
  on:click|stopPropagation={showDropdown}
  hidden={expanded}>
  <div class="selector-avatar typo-overflow-ellipsis">
    <Avatar
      avatarFallback={currentSelectedPeer.identity.avatarFallback}
      dataCy={`avatar-${currentSelectedPeer.identity.metadata.handle}`}
      size="small"
      style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
      variant="circle" />
    <p class="typo-text-bold typo-overflow-ellipsis">
      {currentSelectedPeer.identity.metadata.handle || currentSelectedPeer.identity.shareableEntityIdentifier}
    </p>
    <p>
      {#if maintainers.includes(currentSelectedPeer.identity.urn)}
        <Badge style="margin-left: 0.5rem" variant={BadgeType.Maintainer} />
      {/if}
    </p>
  </div>
  <div class="selector-expand">
    <Icon.ChevronUpDown
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
</div>
<div class="peer-dropdown-container" bind:this={dropdown}>
  <div class="peer-dropdown" hidden={!expanded}>
    {#each revisions as repo}
      <div
        class="peer"
        class:selected={repo.identity.peerId == currentSelectedPeer.identity.peerId}
        data-peer-handle={repo.identity.metadata.handle}>
        <div
          style="display: flex;"
          on:click={() => selectPeer(repo.identity.peerId)}>
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
            {#if maintainers.includes(repo.identity.urn)}
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
  </div>
</div>
