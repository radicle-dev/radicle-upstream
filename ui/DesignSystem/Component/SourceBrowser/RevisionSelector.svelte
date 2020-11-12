<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { isExperimental } from "../../../src/ipc";
  import { RevisionType } from "../../../src/source";
  import type { Branch, Revision, Revisions, Tag } from "../../../src/source";

  import { Icon } from "../../Primitive";
  import Overlay from "../Overlay.svelte";

  export let expanded: boolean = false;
  export let selected: Branch | Tag;
  export let revisions: Revisions;

  const dispatch = createEventDispatcher();
  const hide = () => {
    expanded = false;
  };
  const select = (revision: Revision) => {
    dispatch("select", revision);
    selected = revision as Branch | Tag;
    hide();
  };
  const toggle = () => (expanded = !expanded);
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

<Overlay {expanded} on:{hide}>
  <div
    class="revision-selector"
    data-cy="revision-selector"
    data-revision={selected.name}
    hidden={expanded}
    on:click={toggle}>
    <div class="selector-avatar typo-overflow-ellipsis">
      <div style="display: flex; overflow: hidden;">
        {#if selected.type === RevisionType.Branch}
          <Icon.Branch
            dataCy="branch-icon"
            style="vertical-align: bottom; fill: var(--color-foreground-level-4);
          flex-shrink: 0;" />
        {:else if selected.type === RevisionType.Tag}
          <Icon.Label
            dataCy="tag-icon"
            style="vertical-align: bottom; fill: var(--color-foreground-level-4);
          flex-shrink: 0;" />
        {/if}
        <p class="revision-name typo-overflow-ellipsis">{selected.name}</p>
      </div>
    </div>
    <Icon.ChevronUpDown
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
  <div class="revision-dropdown-container">
    <div class="revision-dropdown" hidden={!expanded}>
      <ul>
        {#each revisions.branches as { name }}
          <li
            class="branch"
            class:selected={selected.type === RevisionType.Branch && selected.name === name}
            data-branch={name}
            on:click|stopPropagation={() => select({
                type: RevisionType.Branch,
                name: name,
              })}>
            <Icon.Branch
              dataCy="branch-icon"
              style="vertical-align: bottom; fill:
            var(--color-foreground-level-4)" />
            <span class="revision-name typo-text">{name}</span>
          </li>
        {/each}
        {#if isExperimental()}
          {#each revisions.tags as { name }}
            <li
              class="tag"
              class:selected={selected.type === RevisionType.Tag && selected.name === name}
              data-tag={name}
              on:click|stopPropagation={() => select({
                  type: RevisionType.Tag,
                  name: name,
                })}>
              <Icon.Label
                dataCy="tag-icon"
                style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
              <span class="revision-name typo-text">{name}</span>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
  </div>
</Overlay>
