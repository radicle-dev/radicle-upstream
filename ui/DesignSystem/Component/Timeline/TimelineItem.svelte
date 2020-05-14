<script>
  import {
    Avatar,
    Button,
    Text,
    Icon,
    Input,
  } from "../../../DesignSystem/Primitive";

  export let item = { variant: "comment" }; // timestamp | open-issue | close-issue | reopen-issue | comment
  const avatarFallback2 = {
    background: {
      r: 122,
      g: 112,
      b: 90,
    },
    emoji: "💡",
  };
</script>

<style>
  li {
    display: flex;
    flex: 1;
    padding: 0 16px 0 48px;
    height: 48px;
    position: relative;
  }
  li::before {
    position: absolute;
    top: 24px;
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
  .comment {
    padding-left: 0;
  }
  .comment-section {
    flex: 1;
  }
  .comment-meta {
    display: flex;
    justify-content: space-between;
    margin-top: 16px;
  }
  .actions {
    display: flex;
  }
</style>

{#if item.variant === 'timestamp'}
  <li class={item.variant}>
    <Text style="color: var(--color-foreground-level-4)">{item.value}</Text>
  </li>
{:else if item.variant === 'open-issue'}
  <li class={item.variant}>
    <Icon.Issue />
    <Avatar
      style="margin: 0 8px; height: 24px;"
      size="small"
      variant="circle"
      imageUrl={item.user.avatar_url}
      title={item.user.handle} />
    <Text style="color: var(--color-foreground-level-6);">
      opened the issue
    </Text>
    <Text style="color: var(--color-foreground-level-5); margin-left: 8px;">
      {item.time_ago}
    </Text>
  </li>
{:else if item.variant === 'close-issue'}
  <li class={item.variant}>
    <Icon.CloseIssue />
    <Avatar
      style="margin: 0 8px; height: 24px;"
      size="small"
      variant="circle"
      imageUrl={item.user.avatar_url}
      title={item.user.handle} />
    <Text style="color: var(--color-foreground-level-6);">
      closed the issue
    </Text>
    <Text style="color: var(--color-foreground-level-5); margin-left: 8px;">
      {item.time_ago}
    </Text>
  </li>
{:else if item.variant === 'reopen-issue'}
  <li class={item.variant}>
    <Icon.Issue />
    <Avatar
      style="margin: 0 8px; height: 24px;"
      size="small"
      variant="circle"
      imageUrl={item.user.avatar_url}
      title={item.user.handle} />
    <Text style="color: var(--color-foreground-level-6);">
      reopened the issue
    </Text>
    <Text style="color: var(--color-foreground-level-5); margin-left: 8px;">
      {item.time_ago}
    </Text>
  </li>
{:else if item.variant === 'comment'}
  <li class={item.variant}>
    <Avatar
      style="margin-right: 16px"
      size="regular"
      variant="circle"
      avatarFallback={avatarFallback2} />
    <div class="comment-section">
      <Input.Text placeholder="Leave a comment" style="flex: 1" />
      <div class="comment-meta">
        <Text
          variant="tiny"
          style="margin-left: 12px; color: var(--color-foreground-level-4);">
          Markdown supported
        </Text>
        <div class="actions">
          <Button variant="vanilla">Close Issue</Button>
          <Button style="margin-left: 16px;">Comment</Button>
        </div>
      </div>
    </div>
  </li>
{/if}
