<script>
  import { link } from "svelte-spa-router";
  import { format } from "timeago.js";

  import * as path from "../../src/path.ts";
  import { commits as store, formatTime } from "../../src/source.ts";

  import { Remote } from "../../DesignSystem/Component";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";

  export let params = null;

  const projectId = params.id;
</script>

<style>
  .commits-page {
    margin: 0 auto 6rem;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 2rem var(--content-padding) 0;
    position: relative;
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
    height: 3rem;
    display: block;
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
  <Remote {store} let:data={histories}>
    {#each histories as history}
      <div class="commit-group">
        <header>
          <p>{formatTime(history.time * 1000)}</p>
        </header>
        <ul>
          {#each history.commits as commit}
            <li class="commit">
              <a href={path.projectCommit(projectId, commit.sha1)} use:link>
                <CommitTeaser
                  {projectId}
                  user={{ username: commit.author.name, avatar: commit.author.avatar }}
                  commitMessage={commit.summary}
                  commitSha={commit.sha1}
                  timestamp={format(commit.committerTime * 1000)}
                  style="background: none; --commit-message-color:
                  var(--color-foreground-level-6); --commit-sha-color:
                  var(--color-foreground)" />
              </a>
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  </Remote>
</div>
