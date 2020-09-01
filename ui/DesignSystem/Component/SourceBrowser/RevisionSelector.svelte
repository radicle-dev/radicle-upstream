<script>
  import { createEventDispatcher, getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../../src/path.ts";
  import { RevisionType } from "../../../src/source.ts";
  import { BadgeType } from "../../../src/badge.ts";

  import { Avatar, Icon } from "../../Primitive";
  import { Tooltip, Badge } from "../../Component";

  export let currentRevision = null;
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
  const selectRevision = (peerId, revision) => {
    dispatch("select", { revision, peerId });
    hideDropdown();
  };
</script>

<style>
  .revision-name {
    color: var(--color-foreground-level-6);
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }
  .revision-selector {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    padding: 0.5rem;
    display: flex;
    cursor: pointer;
    justify-content: space-between;
  }
  .revision-selector:hover {
    color: var(--color-foreground);
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);
  }
  .revision-selector[hidden] {
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
  .revision-dropdown-container {
    display: flex;
    position: absolute;
    top: 0px;
    left: 0px;
    min-width: 100%;
  }
  .revision-dropdown {
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
  .branch,
  .tag {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    cursor: pointer;
    overflow-x: hidden;
    user-select: none;
  }
  .branch:hover,
  .tag:hover {
    background: var(--color-foreground-level-1);
  }
  .branch.selected,
  .branch.selected:hover,
  .tag.selected,
  .tag.selected:hover {
    background-color: var(--color-foreground-level-2);
  }
  .revision-dropdown ul:last-child li {
    border-radius: 0 0 3px 3px;
  }
  .open-profile {
    display: flex;
    justify-content: center;
    cursor: pointer;
  }
</style>

<svelte:window on:click={handleClick} />
<div
  class="revision-selector"
  data-cy="revision-selector"
  data-revision={currentRevision.name}
  on:click|stopPropagation={showDropdown}
  hidden={expanded}>
  <div class="selector-avatar typo-overflow-ellipsis">
    <div style="display: flex; overflow: hidden;">
      {#if currentRevision.type === RevisionType.Branch}
        <Icon.Branch
          dataCy="branch-icon"
          style="vertical-align: bottom; fill: var(--color-foreground-level-4);
          flex-shrink: 0;" />
      {:else}
        <Icon.Label
          dataCy="tag-icon"
          style="vertical-align: bottom; fill: var(--color-foreground-level-4);
          flex-shrink: 0;" />
      {/if}
      <p class="revision-name typo-overflow-ellipsis">{currentRevision.name}</p>
    </div>
    <Avatar
      avatarFallback={currentSelectedPeer.identity.avatarFallback}
      dataCy={`avatar-${currentSelectedPeer.identity.metadata.handle}`}
      size="small"
      style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
      variant="circle" />
  </div>
  <div class="selector-expand">
    <Icon.ChevronUpDown
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
</div>
<div class="revision-dropdown-container" bind:this={dropdown}>
  <div class="revision-dropdown" hidden={!expanded}>
    {#each revisions as repo}
      <div class="peer">
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
      <ul>
        {#each repo.branches as branch}
          <li
            class="branch typo-overflow-ellipsis"
            class:selected={currentRevision.name === branch && currentSelectedPeer.identity.peerId === repo.identity.peerId}
            data-repo-handle={repo.identity.metadata.handle}
            data-branch={branch}
            on:click|stopPropagation={() => selectRevision(
                repo.identity.peerId,
                {
                  type: RevisionType.Branch,
                  peerId: repo.identity.peerId,
                  name: branch,
                }
              )}>
            <Icon.Branch
              dataCy="branch-icon"
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
            <span style="line-height: 1.5rem">{branch}</span>
          </li>
        {/each}
        {#each repo.tags as tag}
          <li
            class="tag typo-overflow-ellipsis"
            data-repo-handle={repo.identity.metadata.handle}
            class:selected={currentRevision.name === tag && currentSelectedPeer.identity.peerId === repo.identity.peerId}
            data-tag={tag}
            on:click|stopPropagation={() => selectRevision(
                repo.identity.peerId,
                {
                  type: RevisionType.Tag,
                  name: tag,
                }
              )}>
            <Icon.Label
              dataCy="tag-icon"
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
            <span style="line-height: 1.5rem">{tag}</span>
          </li>
        {/each}
      </ul>
    {/each}
  </div>
</div>
