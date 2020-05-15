<script>
  import { Icon, Title, Text } from "../../../DesignSystem/Primitive";

  export let issue = null;
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
    {#if issue.open}
      <Icon.Issue style="margin-right: 12px; fill: var(--color-positive)" />
      <div>
        <Title>{issue.title}</Title>
        <Text style="color: var(--color-foreground-level-5); padding-top: 6px">
          Opened {issue.created} ago by {issue.author}
        </Text>
      </div>
    {:else}
      <Icon.Issue style="margin-right: 12px; fill: var(--color-negative)" />
      <div>
        <Title>{issue.title}</Title>
        <Text style="color: var(--color-foreground-level-5); padding-top: 6px">
          Created by {issue.author} closed {issue.closed} ago
        </Text>
      </div>
    {/if}

  </div>
  <div class="right">
    {#if issue.replies > 0}
      <div class="reply-count">
        <Icon.Replies />
        <Text
          style="margin-left: 4px;font-family: var(--typeface-bold); color:
          var(--color-foreground-level-5);">
          {issue.replies}
        </Text>
      </div>
    {/if}
    <Text style="color: var(--color-foreground-level-5);">
      Updated {issue.updated} ago
    </Text>
  </div>
</div>
