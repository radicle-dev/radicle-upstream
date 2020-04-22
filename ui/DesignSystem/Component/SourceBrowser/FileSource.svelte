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

  header {
    display: flex;
    background-color: var(--color-foreground-level-1);
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    height: 48px;
    align-items: center;
    padding-left: 13px;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }

  .line-numbers {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-5);
    text-align: center;
    flex: 0 0 49px;
    border-right: 1px solid var(--color-foreground-level-3);
    user-select: none;
  }

  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    padding-left: 8px;
    overflow-x: scroll;
  }

  .container {
    display: flex;
  }
</style>

<CommitTeaser
  {projectId}
  user={{ username: blob.info.lastCommit.author.name, avatar: blob.info.lastCommit.author.avatar }}
  commitMessage={blob.info.lastCommit.summary}
  commitSha={blob.info.lastCommit.sha1}
  timestamp={format(blob.info.lastCommit.committerTime * 1000)}
  style="margin-bottom: 24px" />

<div class="file-source" data-cy="file-source">
  <header>
    <Icon.File />
    {path}
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
