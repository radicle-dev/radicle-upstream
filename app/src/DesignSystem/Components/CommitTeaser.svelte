<script>
  import UserCard from "./UserCard.svelte";
  import { link } from "svelte-spa-router";
  import { Text } from "../Primitives";

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
    border-radius: 2px;
    border: 1px solid var(--color-lightgray);
    white-space: nowrap;
    min-width: var(--content-min-width);
  }

  .align-left {
    display: flex;
    flex: 1;
    align-items: center;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }

  .commitSha {
    color: var(--color-purple);
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <UserCard {user} style="margin-right: 8px" />
    <Text style="color: var(--color-darkgray)">{commitMessage}</Text>
  </div>

  <div class="align-right">
    <Text style="color: var(--color-darkgray)">
      Latest commit
      <a class="commitSha" href="/projects/123/commits/{commitSha}" use:link>
        {commitSha}
      </a>
      {timestamp}
    </Text>
  </div>
</div>
