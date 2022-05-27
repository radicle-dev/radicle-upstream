<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Branch, Tag } from "ui/src/source";

  import { createEventDispatcher } from "svelte";

  import ChevronUpDownIcon from "design-system/icons/ChevronUpDown.svelte";
  import Overlay from "design-system/Overlay.svelte";

  import Entry from "./RevisionSelector/Entry.svelte";

  export let expanded: boolean = false;
  export let loading: boolean = false;
  export let revisions: Array<Branch | Tag>;
  export let selected: Branch | Tag;
  export let defaultBranch: string;
  export let style: string | undefined = undefined;

  $: orderedRevisions = [
    selected,
    ...revisions.filter(rev => {
      if (rev.name === selected.name && rev.type === selected.type) {
        // Don’t show the selected revision again
        return false;
      } else if (rev.type === "tag") {
        // Tags behave differently in radicle-link than people are used to in
        // normal git workflows, hence we're not showing them in the UI.
        return false;
      } else {
        return true;
      }
    }),
  ];

  const dispatch = createEventDispatcher();

  function hide(): void {
    expanded = false;
  }

  function select(revision: Branch | Tag): void {
    dispatch("select", revision);
    selected = revision as Branch | Tag;
    hide();
  }

  function toggle(): void {
    expanded = !expanded;
  }

  function revisionKey(revision: Branch | Tag): string {
    return `${revision.type}-${revision.name}`;
  }
</script>

<style>
  .container {
    position: relative;
  }
  .revision-selector {
    align-items: center;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    cursor: pointer;
    display: flex;
    height: 2.5rem;
    justify-content: space-between;
    overflow: hidden;
    padding: 0 0.5rem;
    user-select: none;
  }
  .revision-selector:hover {
    color: var(--color-foreground);
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-3);
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
    background: var(--color-background);
    border: 1px solid transparent;
    border-radius: 0.5rem;
    box-shadow: var(--elevation-medium);
    height: 100%;
    max-height: 60vh;
    max-width: 30rem;
    min-width: 100%;
    overflow: scroll;
    position: relative;
    z-index: 8;
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

<div class="container" {style}>
  <Overlay {expanded} on:hide={hide}>
    <div
      class="revision-selector button-transition"
      data-cy="revision-selector"
      role="button"
      aria-label="select branch"
      data-revision={selected.name}
      hidden={expanded}
      on:click={toggle}>
      <div class="selector-avatar typo-overflow-ellipsis">
        <Entry
          {loading}
          on:click={toggle}
          revision={selected}
          defaultBranch={selected.name === defaultBranch} />
      </div>
      <ChevronUpDownIcon
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
    <div class="revision-dropdown-container">
      <div
        class="revision-dropdown"
        data-cy="revision-dropdown"
        hidden={!expanded}>
        <ul>
          {#each orderedRevisions as revision (revisionKey(revision))}
            <li>
              <Entry
                {loading}
                on:click={() => select(revision)}
                {revision}
                defaultBranch={revision.name === defaultBranch}
                selected={selected.type === revision.type &&
                  selected.name === revision.name}
                style="padding: 0 0.5rem;" />
            </li>
          {/each}
        </ul>
      </div>
    </div>
  </Overlay>
</div>
