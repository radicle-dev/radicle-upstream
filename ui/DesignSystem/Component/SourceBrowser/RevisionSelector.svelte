<script>
  import { getContext } from "svelte";

  import { isExperimental } from "../../../../native/ipc.js";
  import { RevisionType } from "../../../src/source.ts";

  import Overlay from "../Overlay.svelte";
  import { Icon } from "../../Primitive";

  export let currentRevision = null;
  export let currentPeerId = null;
  export let expanded = false;
  export let revisions = null;

  let currentSelectedPeer;

  const { metadata } = getContext("project");

  $: if (currentPeerId) {
    currentSelectedPeer = revisions.find(rev => {
      return rev.identity.peerId === currentPeerId;
    });
  } else {
    // The API returns a revision list where the first entry is the default
    // peer.
    currentSelectedPeer = revisions[0];
  }

  // initialize currentRevision
  $: if (!currentRevision) {
    currentRevision = {
      type: RevisionType.Branch,
      name: metadata.defaultBranch,
      peerId: currentSelectedPeer ? currentSelectedPeer.identity.peerId : "",
    };
  }

  const toggle = () => (expanded = !expanded);

  const hideDropdown = () => {
    expanded = false;
  };

  const selectRevision = revision => {
    currentRevision = revision;
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
    padding: 0 0.5rem;
    align-items: center;
    height: 2.5rem;
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
  .revision-dropdown-container {
    position: absolute;
    top: -1px;
    min-width: 100%;
  }
  .revision-dropdown {
    position: relative;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
  }
  .branch,
  .tag {
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    align-items: center;
    height: 2.5rem;
    cursor: pointer;
    overflow-wrap: anywhere;
    user-select: none;
    display: flex;
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
  .revision-dropdown li:only-child {
    border-radius: 0.1875rem;
  }
  .revision-dropdown li:first-child:not(:only-child) {
    border-radius: 0.1875rem 0.1875rem 0 0;
  }
  .revision-dropdown li:last-child:not(:only-child) {
    border-radius: 0 0 0.1875rem 0.1875rem;
  }
</style>

<Overlay {expanded} on:hide={hideDropdown}>
  <div
    class="revision-selector"
    data-cy="revision-selector"
    data-revision={currentRevision.name}
    on:click={toggle}
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
        <p class="revision-name typo-overflow-ellipsis">
          {currentRevision.name}
        </p>
      </div>
    </div>
    <Icon.ChevronUpDown
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
  <div class="revision-dropdown-container">
    <div class="revision-dropdown" hidden={!expanded}>
      <ul>
        {#each currentSelectedPeer.branches as branch}
          <li
            class="branch"
            class:selected={currentRevision.name === branch && currentSelectedPeer.identity.peerId === currentSelectedPeer.identity.peerId}
            data-branch={branch}
            on:click|stopPropagation={() => selectRevision({
                type: RevisionType.Branch,
                peerId: currentSelectedPeer.identity.peerId,
                name: branch,
              })}>
            <Icon.Branch
              dataCy="branch-icon"
              style="vertical-align: bottom; fill:
            var(--color-foreground-level-4)" />
            <span class="revision-name typo-text">{branch}</span>
          </li>
        {/each}
        {#if isExperimental()}
          {#each currentSelectedPeer.tags as tag}
            <li
              class="tag"
              class:selected={currentRevision.name === tag && currentSelectedPeer.identity.peerId === currentSelectedPeer.identity.peerId}
              data-tag={tag}
              on:click|stopPropagation={() => selectRevision({
                  type: RevisionType.Tag,
                  name: tag,
                })}>
              <Icon.Label
                dataCy="tag-icon"
                style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
              <span class="revision-name typo-text">{tag}</span>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
  </div>
</Overlay>
