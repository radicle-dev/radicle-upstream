<script>
  import { format } from "timeago.js";
  import { link } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import {
    commits as store,
    fetchCommits,
    formatTime,
  } from "../../src/source.ts";

  import { Remote } from "../../DesignSystem/Component";
  import { Text } from "../../DesignSystem/Primitive";
  import CommitTeaser from "../../DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";

  export let params = null;

  const projectId = params.id;
  const branch = params.branch;

  fetchCommits({ projectId, branch });
</script>

<style>
  .commits-page {
    padding-top: 32px;
    margin-bottom: 64px;
    margin-left: 96px;
    margin-right: 96px;
    min-width: 720px;
    position: relative;
  }
  .commit-group header {
    padding-bottom: 0.75rem;
    padding-left: 1rem;
    color: var(--color-foreground-level-6);
  }
  .commit-group ul {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    margin-bottom: 2rem;
  }
  .commit {
    border-bottom: 1px solid var(--color-foreground-level-3);
    height: 48px;
    display: block;
    padding: 4px 0;
  }
  .commit:first-child {
    border-top-left-radius: 4px;
    border-top-right-radius: 4px;
  }
  .commit:last-child {
    border-bottom: none;
    border-bottom-left-radius: 4px;
    border-bottom-right-radius: 4px;
  }
  .commit:hover {
    background: var(--color-foreground-level-1);
  }
</style>

<div class="commits-page">
  <Remote {store} let:data={histories}>
    {#each histories as history}
      <div class="commit-group">
        <header>
          <Text>{formatTime(history.time * 1000)}</Text>
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
                  var(--color-foreground-level-6)" />
              </a>
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  </Remote>
</div>
