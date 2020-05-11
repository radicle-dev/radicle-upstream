<script>
  import { Icon, Title, Text } from "../../DesignSystem/Primitive";

  export let open = true;
  export let title = null;
  export let author = null;
  export let replies = null;
  export let created = null;
  export let updated = null;
  export let closed = null;
  const iconColor = open ? "var(--color-positive)" : "var(--color-negative)";
</script>

<style>
  .issueCard {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 85px;
    flex: 1;
    cursor: pointer;
    padding: 16px;
  }

  .issueCard:hover {
    background-color: var(--color-foreground-level-1);
  }

  .title {
    display: flex;
  }

  .right {
    display: flex;
    align-items: center;
  }
  .reply-count {
    display: flex;
    margin-right: 24px;
  }
</style>

<div class="issueCard">
  <div class="title">
    <Icon.Issue style="margin-right: 12px; fill: {iconColor}" />
    <div>
      <Title>{title}</Title>
      <Text style="color: var(--color-foreground-level-5); padding-top: 6px">
        {#if open}
          Opened {created} ago by {author}
        {:else}Created by {author} closed {closed} ago{/if}
      </Text>

    </div>
  </div>
  <div class="right">
    {#if replies > 0}
      <div class="reply-count">
        <Icon.Replies />
        <Text
          style="margin-left: 4px;font-family: var(--typeface-bold); color:
          var(--color-foreground-level-5);">
          {replies}
        </Text>
      </div>
    {/if}
    <Text style="color: var(--color-foreground-level-5);">
      Updated {updated} ago
    </Text>
  </div>
</div>
