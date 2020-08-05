<script>
  import { Icon } from "../../../DesignSystem/Primitive";

  export let issue = null;

  const issueCaption = issue => {
    if (issue.open) {
      return `Opened ${issue.created_at} ago by ${issue.author}`;
    } else {
      return `Created by ${issue.author} closed ${issue.closed_at} ago`;
    }
  };

  const issueIconColor = issueOpen => {
    return issueOpen ? "--color-positive" : "--color-negative";
  };
</script>

<style>
  .issue-card {
    display: flex;
    justify-content: space-between;
    flex: 1;
    padding: 16px;
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
      <p class="typo-bold">{issue.title}</p>
      <p style="color: var(--color-foreground-level-5); padding-top: 6px">
        {issueCaption(issue)}
      </p>
    </div>
  </div>
  <div class="right">
    {#if issue.replies > 0}
      <div class="reply-count">
        <Icon.Replies />
        <p
          class="typo-bold"
          style="margin-left: 4px; color: var(--color-foreground-level-5);">
          {issue.replies}
        </p>
      </div>
    {/if}
    <p style="color: var(--color-foreground-level-5);">
      Updated {issue.updated_at} ago
    </p>
  </div>
</div>
