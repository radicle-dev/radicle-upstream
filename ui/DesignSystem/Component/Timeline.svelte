<script>
  import ActionItem from "./Timeline/ActionItem.svelte";
  import CommentItem from "./Timeline/CommentItem.svelte";
  export let startDate = null;
  export let items = null;
  export let style = null;
</script>

<style>
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
  .base-item {
    padding: 0 16px 0 48px;
    height: 48px;
  }
  .comment-item {
    padding-bottom: 24px;
  }
</style>

<ul {style}>
  <li class="base-item">
    <p style="color: var(--color-foreground-level-4)">{startDate}</p>
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
</ul>
