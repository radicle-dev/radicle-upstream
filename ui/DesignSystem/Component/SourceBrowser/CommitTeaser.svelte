<script>
  import { link } from "svelte-spa-router";
  import { Text } from "../../Primitive";
  import UserCard from "../UserCard.svelte";
  import * as path from "../../../lib/path.js";

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
    height: 36px;
    padding: 0 12px 0 12px;
    white-space: nowrap;
    min-width: var(--content-min-width);
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
    color: var(--color-foreground-level-6);
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    color: var(--color-secondary);
    font-family: var(--typeface-mono-regular);
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <UserCard {user} style="margin-right: 8px" />

    <p class="commit-message">{commitMessage}</p>
  </div>

  <div class="align-right">
    <Text style="color: var(--color-foreground-level-6)">
      Latest commit
      <a
        class="commit-sha"
        href={path.projectCommit(projectId, commitSha)}
        use:link>
        {commitSha.substring(0, 7)}
      </a>
      {timestamp}
    </Text>
  </div>
</div>
