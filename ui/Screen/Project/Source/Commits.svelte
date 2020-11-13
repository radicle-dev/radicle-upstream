<script lang="typescript">
  import { format } from "timeago.js";

  import { selectCommit, store } from "../../../src/screen/project/source";
  import { formatTime } from "../../../src/source";
  import type { Commit } from "../../../src/source";

  import { Remote } from "../../../DesignSystem/Component";
  import CommitTeaser from "../../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";

  const onSelect = (commit: Commit) => {
    selectCommit(commit);
  };
</script>

<style>
  .commits-page {
    margin: 0 auto 6rem;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 2rem var(--content-padding) 0;
  }
  .commit-group header {
    padding-bottom: 0.75rem;
    padding-left: 1rem;
    color: var(--color-foreground-level-6);
  }
  .commit-group ul {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
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
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }
  .commit:last-child {
    border-bottom: none;
    border-bottom-left-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
  }
  .commit:hover {
    background: var(--color-foreground-level-1);
  }
</style>

<div class="commits-page" data-cy="commits-page">
  <Remote {store} let:data={{ history }}>
    {#each history.history as group (group.time)}
      <div class="commit-group">
        <header>
          <p>{formatTime(group.time * 1000)}</p>
        </header>
        <ul>
          {#each group.commits as commit (commit.sha1)}
            <li class="commit" on:click={() => onSelect(commit)}>
              <CommitTeaser
                message={commit.summary}
                sha={commit.sha1}
                style="background: none; --commit-message-color:
                var(--color-foreground-level-6); --commit-sha-color:
                var(--color-foreground)"
                timestamp={format(commit.committerTime * 1000)}
                user={commit.author} />
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  </Remote>
</div>
