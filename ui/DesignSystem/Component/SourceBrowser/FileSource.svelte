<script>
  import { format } from "timeago.js";

  import { Icon } from "../../Primitive";
  import CommitTeaser from "./CommitTeaser.svelte";

  export let blob = null;
  export let path = null;
  export let projectId = null;
</script>

<style>
  .file-source {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header .file-header {
    display: flex;
    font-weight: 600;
    font-size: 1rem;
    height: 3rem;
    align-items: center;
    padding-left: 13px;
    color: var(--color-foreground);
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  header .file-name {
    margin-left: 0.5rem;
  }

  header .commit-header {
    height: 3rem;
    background-color: var(--color-secondary-level-1);
  }

  .line-numbers {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-5);
    text-align: center;
    flex: 0 0 49px;
    user-select: none;
  }

  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    padding-left: 8px;
    overflow-x: scroll;
  }

  .code,
  .line-numbers {
    padding: 0.75rem 0;
  }

  .container {
    display: flex;
  }
</style>

<div class="file-source" data-cy="file-source">
  <header>
    <div class="file-header">
      <Icon.File />
      <span class="file-name">{path}</span>
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
