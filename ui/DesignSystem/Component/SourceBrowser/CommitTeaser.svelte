<script>
  import { link } from "svelte-spa-router";

  import * as path from "../../../src/path.ts";

  import { Icon } from "../../Primitive";

  export let projectId = null;
  export let commitMessage = null;
  export let timestamp = null;
  export let commitSha = null;
  export let user = null;

  export let style = null;
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0 12px 0 8px;
    white-space: nowrap;
    min-width: var(--content-min-width);
    border-radius: 4px;
    background-color: var(--color-secondary-level-1);
  }

  .align-left {
    display: flex;
    flex: 1;
    align-items: center;
    max-width: 64%;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }

  .commit-message {
    color: var(--commit-message-color, var(--color-secondary));
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    padding: 0 8px 0 4px;
    color: var(--commit-sha-color, var(--color-secondary));
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <Icon.Commit style="fill: var(--color-secondary)" />
    <a
      class="commit-sha typo-text-small-mono"
      href={path.projectCommit(projectId, commitSha)}
      use:link>
      {commitSha.substring(0, 7)}
    </a>
    <p class="commit-message typo-small">{commitMessage}</p>
  </div>

  <div class="align-right">
    <p
      class="small-bold"
      style="margin-right: 8px; color: var(--color-foreground-level-6)">
      {user.username}
    </p>
    <p class="typo-small" style="color: var(--color-foreground-level-6)">
      {timestamp}
    </p>
  </div>
</div>
