<script>
  import { format } from "timeago.js";
  import { link } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { commits as store, fetchCommits } from "../../src/source.ts";

  import { Remote } from "../../DesignSystem/Component";
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
  }
  .commit-group {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
  }
  .commit {
    border-bottom: 1px solid var(--color-foreground-level-2);
    height: 48px;
    display: block;
    padding: 4px 0;
  }
  .commit:last-child {
    border-bottom: none;
  }
  .commit:hover {
    background-color: var(--color-foreground-level-1);
  }
</style>

<div class="commits-page">
  <Remote {store} let:data={commits}>
    <ul class="commit-group">
      {#each commits as commit}
        <li class="commit">
          <a href={path.projectCommit(projectId, commit.sha1)} use:link>
            <CommitTeaser
              {projectId}
              user={{ username: commit.author.name, avatar: commit.author.avatar }}
              commitMessage={commit.summary}
              commitSha={commit.sha1}
              timestamp={format(commit.committerTime * 1000)}
              style="background: none" />
          </a>
        </li>
      {/each}
    </ul>
  </Remote>
</div>
