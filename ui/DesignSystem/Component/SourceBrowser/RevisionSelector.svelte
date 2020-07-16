<script>
  import { createEventDispatcher } from "svelte";

  import { RevisionType } from "../../../src/source.ts";

  import { Avatar, Icon, Text } from "../../Primitive";

  export let currentRevision = null;
  export let currentPeerId = null;
  export let expanded = false;
  export let revisions = null;
  export let style = null;

  let currentSelectedPeer;

  $: if (currentPeerId) {
    currentSelectedPeer = revisions.find(rev => {
      return rev.identity.id === currentPeerId;
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

  const dispatch = createEventDispatcher();
  const selectRevision = (peerId, revision) => {
    dispatch("select", { revision, peerId });
    hideDropdown();
  };
</script>

<style>
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
    overflow: hidden;
    text-overflow: ellipsis;
    margin-right: 0.5rem;
    display: flex;
  }
  .selector-expand {
    align-self: flex-end;
  }
  .revision-dropdown-container {
    display: flex;
    position: absolute;
    top: 0px;
    left: 0px;
  }
  .revision-dropdown {
    position: relative;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 40rem;
  }
  .peer {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    user-select: none;
  }
  .branch,
  .tag {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    cursor: pointer;
    overflow-x: hidden;
    user-select: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .branch:hover,
  .tag:hover {
    color: var(--color-foreground);
    background: var(--color-foreground-level-2);
  }
  .revision-dropdown ul:last-child li {
    border-radius: 0 0 3px 3px;
  }
</style>

<svelte:window on:click={handleClick} />
<div
  class="revision-selector"
  data-cy="revision-selector"
  data-revision={currentRevision}
  on:click|stopPropagation={showDropdown}
  hidden={expanded}>
  <div class="selector-avatar">
    <Avatar
      avatarFallback={currentSelectedPeer.identity.avatarFallback}
      size="small"
      style="display: flex; justify-content: flex-start; margin-right: 8px;"
      variant="circle" />
    <Text
      style="white-space: nowrap; text-overflow: ellipsis; overflow: hidden;">
      {currentRevision}
    </Text>
  </div>
  <div class="selector-expand">
    <Icon.Expand
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
</div>
<div class="revision-dropdown-container" bind:this={dropdown}>
  <div class="revision-dropdown" hidden={!expanded} {style}>
    {#each revisions as repo}
      <div class="peer">
        <Avatar
          avatarFallback={repo.identity.avatarFallback}
          style="display: flex; justify-content: flex-start; margin-right: 8px;"
          size="small"
          variant="circle" />
        <Text
          style="white-space: nowrap; text-overflow: ellipsis; overflow: hidden;">
          {repo.identity.shareableEntityIdentifier}
        </Text>
      </div>
      <ul>
        {#each repo.branches as branch}
          <li
            class="branch"
            data-repo-handle={repo.identity.metadata.handle}
            data-branch={branch}
            on:click|stopPropagation={() => selectRevision(repo.identity.id, {
                type: RevisionType.Branch,
                peerId: repo.identity.id,
                name: branch,
              })}>
            <Icon.Branch
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
            <span style="line-height: 1.5rem">{branch}</span>
          </li>
        {/each}
        {#each repo.tags as tag}
          <li
            class="tag"
            data-repo-handle={repo.identity.metadata.handle}
            data-tag={tag}
            on:click|stopPropagation={() => selectRevision(repo.identity.id, {
                type: RevisionType.Tag,
                name: tag,
              })}>
            <Icon.Commit
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
            <span style="line-height: 1.5rem">{tag}</span>
          </li>
        {/each}
      </ul>
    {/each}
  </div>
</div>
