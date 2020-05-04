<script>
  import { link } from "svelte-spa-router";
  import { Text, Icon } from "../../Primitive";
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
    color: var(--color-secondary);
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    color: var(--color-secondary);
    font-family: var(--typeface-mono-bold);
    padding: 0 8px 0 4px;
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <Icon.Commit style="fill: var(--color-secondary)" />
    <a
      class="commit-sha"
      href={path.projectCommit(projectId, commitSha)}
      use:link>
      {commitSha.substring(0, 7)}
    </a>
    <p class="commit-message">{commitMessage}</p>
  </div>

  <div class="align-right">
    <UserCard
      {user}
      style="margin-right: 8px; color: var(--color-foreground-level-6)" />
    <Text style="color: var(--color-foreground-level-6)">{timestamp}</Text>
  </div>
</div>
