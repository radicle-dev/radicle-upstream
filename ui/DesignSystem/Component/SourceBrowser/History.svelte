<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type {
    CommitHeader,
    GroupedCommitsHistory,
  } from "../../../src/source";

  import CommitTeaser from "./CommitTeaser.svelte";

  export let history: GroupedCommitsHistory;

  const dispatch = createEventDispatcher();
  const onSelect = (commit: CommitHeader) => {
    dispatch("select", commit);
  };
</script>

<style>
  .commit-group header {
    padding-bottom: 0.75rem;
    padding-left: 1rem;
    color: var(--color-foreground-level-6);
  }
  .commit-group ul {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    margin-bottom: 2rem;
  }
  .commit {
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    display: block;
    height: 3rem;
    padding: 0.25rem 0;
  }
  .commit:first-child {
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
  }
  .commit:last-child {
    border-bottom: none;
    border-bottom-left-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }
  .commit:hover {
    background: var(--color-foreground-level-1);
  }
</style>

<div data-cy="history">
  {#each history.history as group (group.time)}
    <div class="commit-group" data-cy="commit-group">
      <header>
        <p>{group.time}</p>
      </header>
      <ul>
        {#each group.commits as commit (commit.sha1)}
          <li class="commit" data-cy="commit" on:click={() => onSelect(commit)}>
            <CommitTeaser
              {commit}
              style="background: none; --commit-message-color:
            var(--color-foreground-level-6); --commit-sha-color:
            var(--color-foreground)" />
          </li>
        {/each}
      </ul>
    </div>
  {/each}
</div>
