<script>
  import { Icon, Title, Text } from "../../../DesignSystem/Primitive";

  export let issue = null;

  const issueCaption = (issue) => {
    if (issue.open) {
      return `Opened ${issue.created_at} ago by ${issue.author}`;
    } else {
      return `Created by ${issue.author} closed ${issue.closed_at} ago`;
    }
  };

  const issueIconColor = (issueOpen) => {
    return issueOpen ? "--color-positive" : "--color-negative";
  };
</script>

<style>
  .issue-card {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 85px;
    flex: 1;
    cursor: pointer;
    padding: 16px;
  }

  .issue-card:hover {
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

<div class="issue-card">
  <div class="title">
    <Icon.Issue
      style="margin-right: 12px; fill: var({issueIconColor(issue.open)})" />
    <div>
      <Title>{issue.title}</Title>
      <Text style="color: var(--color-foreground-level-5); padding-top: 6px">
        {issueCaption(issue)}
      </Text>
    </div>
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
      Updated {issue.updated_at} ago
    </Text>
  </div>
</div>
