<script>
  import { format } from "timeago.js";
  import { link } from "svelte-spa-router";

  import { Icon } from "../../Primitive";
  import CommitTeaser from "./CommitTeaser.svelte";

  export let blob = null;
  export let path = null;
  export let rootPath = null;
  export let projectId = null;
  export let projectName = null;
</script>

<style>
  .file-source {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header .file-header {
    display: flex;
    font-family: var(--typeface-medium);
    font-size: 1rem;
    height: 3rem;
    align-items: center;
    padding-left: 13px;
    color: var(--color-foreground);
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  header .file-name {
    margin-left: 0.5rem;
    font-family: var(--typeface-medium);
  }

  header .file-name a {
    color: var(--color-foreground-level-5);
  }

  header .file-name a:hover {
    text-decoration: underline;
  }

  header .commit-header {
    height: 2.5rem;
    margin: 0.5rem;
  }

  .line-numbers {
    font-family: var(--typeface-mono-regular);
    font-size: 16px;
    color: var(--color-foreground-level-4);
    text-align: right;
    user-select: none;
    padding: 0 1rem 0.5rem 1rem;
  }

  .code {
    padding-bottom: 0.5rem;
  }

  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 16px;
    overflow-x: auto;
  }

  .container {
    display: flex;
  }
</style>

<div class="file-source" data-cy="file-source">
  <header>
    <div class="file-header">
      <Icon.File />
      <span class="file-name">
        <a href={rootPath} use:link>{projectName}</a>
        <span>/ {path.split('/').join(' / ')}</span>
      </span>
    </div>
    <div class="commit-header">
      <CommitTeaser
        {projectId}
        user={{ username: blob.info.lastCommit.author.name, avatar: blob.info.lastCommit.author.avatar }}
        commitMessage={blob.info.lastCommit.summary}
        commitSha={blob.info.lastCommit.sha1}
        timestamp={format(blob.info.lastCommit.committerTime * 1000)}
        style="height: 100%" />
    </div>
  </header>
  <div class="container">
    {#if blob.binary}
      ఠ ͟ಠ Binary content.
    {:else}
      <pre class="line-numbers">
        {@html blob.content
          .split('\n')
          .slice(0, -1)
          .map((_, index) => {
            return `${index + 1}`;
          })
          .join('\n')}
      </pre>
      <pre class="code">{blob.content}</pre>
    {/if}
  </div>
</div>
