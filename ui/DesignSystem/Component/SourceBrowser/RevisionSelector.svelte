<script>
  /* import { gql } from "apollo-boost"; */
  /* import { getContext } from "svelte"; */
  /* import { getClient, query } from "svelte-apollo"; */
  import ClickOutside from "svelte-click-outside";

  /* import { revisionStore } from "../../../store/sourceBrowser.js"; */
  /* import { HIDDEN_BRANCHES } from "../../../config.js"; */

  import { Avatar, Icon } from "../../Primitive";

  export let style = "";
  export let expanded = false;

  const toggleDropdown = () => {
    expanded = !expanded;
  };

  const hideDropdown = () => {
    expanded = false;
  };

  /* const ALL_REVISIONS = gql` */
  /*   query($projectId: ID!) { */
  /*     tags(id: $projectId) */
  /*     branches(id: $projectId) */
  /*   } */
  /* `; */

  /* const allRevisions = query(getClient(), { */
  /*   query: ALL_REVISIONS, */
  /*   variables: { projectId: getContext("projectId") } */
  /* }); */

  const mockRevisions = async () => {
    const data = [
      {
        user: {
          avatar: { emoji: "🐯", background: { r: 230, g: 130, b: 230 } },
          handle: "cloudhead"
        },
        branches: ["master"],
        tags: ["v0.1.2"]
      },
      {
        user: {
          avatar: { emoji: "👻", background: { r: 230, g: 230, b: 230 } },
          handle: "rudolfs"
        },
        branches: ["master", "development", "feature/icons"],
        tags: ["v0.1.2"]
      },
      {
        user: {
          avatar: { emoji: "🤡", background: { r: 130, g: 230, b: 230 } },
          handle: "xla"
        },
        branches: ["master", "development"]
      }
    ];
    return { data: data };
  };

  // Set by the view.
  let triggerEl = null;
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

{#await mockRevisions() then result}
  <div
    class="revision-selector"
    bind:this={triggerEl}
    on:click={toggleDropdown}
    hidden={expanded}>
    <div class="selector-avatar">
      <Avatar
        title={result.data[0].user.handle}
        avatarFallback={result.data[0].user.avatar}
        size="small"
        variant="user" />
    </div>
    <div class="selector-branch">{result.data[0].branches[0]}</div>
    <div class="selector-expand">
      <Icon.Expand
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="revision-dropdown-container">
    <ClickOutside
      on:clickoutside={hideDropdown}
      exclude={[triggerEl]}
      useWindow>
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
              <li class="branch">
                <Icon.Branch
                  style="vertical-align: bottom; fill:
                  var(--color-foreground-level-4)" />
                <span style="line-height: 1.5rem">{branch}</span>
              </li>
            {/each}
          </ul>
        {/each}
      </div>
    </ClickOutside>
  </div>
{/await}
