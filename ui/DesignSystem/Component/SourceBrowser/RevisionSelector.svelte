<script>
  import { gql } from "apollo-boost";
  import { getContext } from "svelte";
  import { getClient, query } from "svelte-apollo";

  import { revisionStore } from "../../../store/sourceBrowser.js";
  import { HIDDEN_BRANCHES } from "../../../config.js";

  import { Avatar, Icon } from "../../Primitive";

  export let style = "";
  export let expanded = false;

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

  const selectRevision = (ev, rev) => {
    revisionStore.set(rev);
    hideDropdown();
  };

  const ALL_REVISIONS = gql`
    query($projectId: ID!) {
      tags(id: $projectId)
      branches(id: $projectId)
    }
  `;

  const allRevisions = query(getClient(), {
    query: ALL_REVISIONS,
    variables: { projectId: getContext("projectId") }
  });

  const mockRevisions = async () => {
    const response = await allRevisions;
    const revisions = (await response.result()).data;

    const data = [
      {
        user: {
          avatar: { emoji: "🐯", background: { r: 230, g: 130, b: 230 } },
          handle: "cloudhead"
        },
        branches: revisions.branches,
        tags: ["v0.1.2"]
      },
      {
        user: {
          avatar: { emoji: "👻", background: { r: 230, g: 230, b: 230 } },
          handle: "rudolfs"
        },
        branches: revisions.branches,
        tags: ["v0.1.2"]
      },
      {
        user: {
          avatar: { emoji: "🤡", background: { r: 130, g: 230, b: 230 } },
          handle: "xla"
        },
        branches: revisions.branches
      }
    ];
    return { data: data };
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
{#await mockRevisions() then result}
  <div
    class="revision-selector"
    on:click|stopPropagation={showDropdown}
    hidden={expanded}>
    <div class="selector-avatar">
      <Avatar
        title={result.data[0].user.handle}
        avatarFallback={result.data[0].user.avatar}
        size="small"
        variant="user" />
    </div>
    <div class="selector-branch">{$revisionStore}</div>
    <div class="selector-expand">
      <Icon.Expand
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="revision-dropdown-container" bind:this={dropdown}>
    <div class="revision-dropdown" hidden={!expanded} {style}>
      {#each result.data as repo}
        <div class="user">
          <!-- TODO(cloudhead): text color should be `color-foreground-level-6`,
          but `Avatar` doesn't allow overwriting. -->
          <Avatar
            title={repo.user.handle}
            avatarFallback={repo.user.avatar}
            size="small"
            variant="user" />
        </div>
        <ul>
          {#each repo.branches as branch}
            {#if !HIDDEN_BRANCHES.includes(branch)}
              <li
                class="branch"
                on:click|stopPropagation={ev => selectRevision(ev, branch)}>
                <Icon.Branch
                  style="vertical-align: bottom; fill:
                  var(--color-foreground-level-4)" />
                <span style="line-height: 1.5rem">{branch}</span>
              </li>
            {/if}
          {/each}
        </ul>
      {/each}
    </div>
  </div>
{/await}
