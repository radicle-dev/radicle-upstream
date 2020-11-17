<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Branch, Revision, Tag } from "../../../src/source";

  import { Icon } from "../../Primitive";
  import Overlay from "../Overlay.svelte";

  import Entry from "./RevisionSelector/Entry.svelte";

  export let expanded: boolean = false;
  export let loading: boolean = false;
  export let revisions: [Branch | Tag];
  export let selected: Branch | Tag;

  const dispatch = createEventDispatcher();
  const hide = () => {
    expanded = false;
  };
  const select = (revision: Revision) => {
    dispatch("select", revision);
    selected = revision as Branch | Tag;
    hide();
  };
  const toggle = () => {
    expanded = !expanded;
  };
</script>

<style>
  .revision-selector {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    align-items: center;
    height: 2.5rem;
    display: flex;
    cursor: pointer;
    justify-content: space-between;
    user-select: none;
  }
  .revision-selector:hover {
    color: var(--color-foreground);
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);
  }
  .revision-selector[hidden] {
    visibility: hidden;
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

<Overlay {expanded} on:hide={hide}>
  <div
    class="revision-selector"
    data-cy="revision-selector"
    data-revision={selected.name}
    hidden={expanded}
    on:click={toggle}>
    <div class="selector-avatar typo-overflow-ellipsis">
      <Entry {loading} on:click={toggle} revision={selected} />
    </div>
    <Icon.ChevronUpDown
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
  <div class="revision-dropdown-container">
    <div
      class="revision-dropdown"
      data-cy="revision-dropdown"
      hidden={!expanded}>
      <ul>
        {#each revisions as revision}
          <li>
            <Entry
              {loading}
              on:click={() => select(revision)}
              {revision}
              selected={selected.type === revision.type && selected.name === revision.name} />
          </li>
        {/each}
      </ul>
    </div>
  </div>
</Overlay>
