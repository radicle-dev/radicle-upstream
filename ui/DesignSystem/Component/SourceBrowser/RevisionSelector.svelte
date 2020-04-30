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
    // Any click *outside* the dropdown should hide the dropdown.
    if (dropdown !== ev.target && !dropdown.contains(ev.target)) {
      hideDropdown();
    }
  };

  const dispatch = createEventDispatcher();
  const selectRevision = (_repo, rev) => {
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
  .revision-selector:hover {
    box-shadow: 0px 0px 0px 1px var(--color-foreground-level-3);
    color: var(--color-foreground);
    background-color: var(--color-foreground-level-2);
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
    color: var(--color-foreground-level-6);
    margin-right: 0.5rem;
  }
  .selector-expand {
    align-self: flex-end;
  }
  .revision-dropdown-container {
    position: absolute;
    top: 0px;
    left: 0px;
    width: 100%;
  }
  .revision-dropdown {
    position: relative;
    background: white;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-medium),
      0px 0px 0px 1px var(--color-foreground-level-3);
    z-index: 8;
  }
  .user {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    display: inline-block;
    user-select: none;
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
    color: var(--color-foreground);
    background: var(--color-foreground-level-2);
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
      title={revisions[0].identity.metadata.handle}
      avatarFallback={revisions[0].identity.avatarFallback}
      size="small"
      style="--title-color: var(--color-foreground-level-6);"
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
        <Avatar
          title={repo.identity.metadata.handle}
          avatarFallback={repo.identity.avatarFallback}
          size="small"
          variant="user"
          style="--title-color: var(--color-foreground-level-6);" />
      </div>
      <ul>
        {#each repo.branches as branch}
          <li
            class="branch"
            data-repo-handle={repo.identity.metadata.handle}
            data-branch={branch}
            on:click|stopPropagation={() => selectRevision(repo.identity, branch)}>
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
            on:click|stopPropagation={() => selectRevision(repo.identity, tag)}>
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
