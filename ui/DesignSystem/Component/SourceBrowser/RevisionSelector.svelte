<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { isExperimental } from "../../../../native/ipc.js";
  import * as source from "../../../src/source";

  import Overlay from "../Overlay.svelte";
  import { Icon } from "../../Primitive";

  export let revisions: source.PeerRevisions;
  export let currentRevision: source.Branch | source.Tag | undefined;

  if (!currentRevision) currentRevision = revisions.branches[0];

  let expanded: boolean = false;
  const toggle = () => (expanded = !expanded);
  const hideDropdown = () => {
    expanded = false;
  };

  const dispatch = createEventDispatcher();

  const selectRevision = (revision: source.Branch | source.Tag) => {
    dispatch("select", { revision });
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
    position: absolute;
    top: 0px;
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
  .branch,
  .tag {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
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
  .revision-dropdown ul:last-child li {
    border-radius: 0 0 3px 3px;
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
        {#if currentRevision.type === source.RevisionType.Branch}
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
    <div class="selector-expand">
      <Icon.ChevronUpDown
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="revision-dropdown-container">
    <div class="revision-dropdown" hidden={!expanded}>
      <ul>
        {#each revisions.branches as branch}
          <li
            class="branch"
            class:selected={currentRevision.name === branch.name && currentRevision.peerId === branch.peerId}
            data-branch={branch.name}
            on:click={() => selectRevision(branch)}>
            <Icon.Branch
              dataCy="branch-icon"
              style="vertical-align: bottom; fill:
            var(--color-foreground-level-4)" />
            <span class="revision-name typo-text">{branch.name}</span>
          </li>
        {/each}
        {#if isExperimental()}
          {#each revisions.tags as tag}
            <li
              class="tag"
              class:selected={currentRevision.name === tag.name && currentRevision.peerId === tag.peerId}
              data-tag={tag.name}
              on:click={() => selectRevision(tag)}>
              <Icon.Label
                dataCy="tag-icon"
                style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
              <span class="revision-name typo-text">{tag.name}</span>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
  </div>
</Overlay>
