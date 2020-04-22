<script>
  import { createEventDispatcher } from "svelte";

  import { Avatar, Icon } from "../../Primitive";

  export let currentRevision = null;
  export let expanded = false;
  export let revisions = null;
  export let style = "";

  // Dropdown element. Set by the view.
  let dropdown = null;

  const showDropdown = () => {
    expanded = true;
  };

  const hideDropdown = () => {
    expanded = false;
  };

  const handleClick = ev => {
    if (dropdown !== ev.target && !dropdown.contains(ev.target)) {
      hideDropdown();
    }
  };

  const dispatch = createEventDispatcher();
  const selectRevision = (ev, rev) => {
    dispatch("select", rev);
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
  }
  .revision-selector[hidden] {
    visibility: hidden;
  }
  .selector-avatar {
    flex: 1;
    margin-right: 0.5rem;
  }
  .selector-branch {
    flex: 1;
    text-overflow: ellipsis;
    overflow-x: hidden;
  }
  .selector-expand {
    align-self: flex-end;
  }
  .revision-dropdown-container {
    position: absolute;
    top: -1px; /* We set this to `-1px` to offset the difference in border */
    left: -1px; /* width between the hidden and expanded states. */
    width: 100%;
  }
  .revision-dropdown {
    position: relative;
    background: white;
    border: 2px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.12), 0px 0px 1px rgba(0, 0, 0, 0.12);
    z-index: 8;
  }
  .user {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    display: inline-block;
  }
  .branch {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    cursor: pointer;
    overflow-x: hidden;
  }
  .branch:hover {
    background: var(--color-foreground-level-1);
  }
</style>

<svelte:window on:click={handleClick} />
<div
  class="revision-selector"
  on:click|stopPropagation={showDropdown}
  hidden={expanded}>
  <div class="selector-avatar">
    <Avatar
      title={revisions[0].identity.metadata.handle}
      avatarFallback={revisions[0].identity.avatarFallback}
      size="small"
      variant="user" />
  </div>
  <div class="selector-branch">{currentRevision}</div>
  <div class="selector-expand">
    <Icon.Expand
      style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
  </div>
</div>
<div class="revision-dropdown-container" bind:this={dropdown}>
  <div class="revision-dropdown" hidden={!expanded} {style}>
    {#each revisions as repo}
      <div class="user">
        <!-- TODO(cloudhead): text color should be `color-foreground-level-6`,
        but `Avatar` doesn't allow overwriting. -->
        <Avatar
          title={repo.identity.metadata.handle}
          avatarFallback={repo.identity.avatarFallback}
          size="small"
          variant="user" />
      </div>
      <ul>
        {#each repo.branches as branch}
          <li
            class="branch"
            on:click|stopPropagation={ev => selectRevision(ev, branch)}>
            <Icon.Branch
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-4)" />
            <span style="line-height: 1.5rem">{branch}</span>
          </li>
        {/each}
      </ul>
    {/each}
  </div>
</div>
