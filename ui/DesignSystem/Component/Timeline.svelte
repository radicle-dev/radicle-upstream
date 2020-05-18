<script>
  import { Text } from "../../DesignSystem/Primitive";
  import ActionItem from "./Timeline/ActionItem.svelte";
  import CommentItem from "./Timeline/CommentItem.svelte";
  import CommentAction from "./Timeline/CommentAction.svelte";
  export let startDate = null;
  export let items = null;

  // fake current user
  const currentUser = {
    handle: "julien",
    avatar_url:
      "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
  };
</script>

<style>
  ul {
    margin-top: 24px;
    margin-bottom: 64px;
  }

  li {
    display: flex;
    flex: 1;
    position: relative;
  }
  li::before {
    position: absolute;
    height: 24px;
    bottom: 0;
    left: 59px;
    display: block;
    width: 2px;
    content: "";
    background-color: var(--color-foreground-level-2);
  }
  li:last-child::before {
    display: none;
  }
  .base-item {
    padding: 0 16px 0 48px;
    height: 48px;
  }
  .comment-item {
    padding-bottom: 24px;
  }
</style>

<ul>
  <li class="base-item">
    <Text style="color: var(--color-foreground-level-4)">{startDate}</Text>
  </li>
  {#each items as item}
    {#if item.variant === 'comment'}
      <li class="comment-item">
        <CommentItem {item} />
      </li>
    {:else}
      <li class="base-item">
        <ActionItem {item} />
      </li>
    {/if}
  {/each}
  <CommentAction user={currentUser} />
</ul>
