<script>
  import { Text, Icon, Avatar } from "../../../DesignSystem/Primitive";

  export let item = { variant: "comment" }; // timestamp | open-issue | close-issue | reopen-issue | comment
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
    <Text>opened the issue</Text>
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
    <Text>closed the issue</Text>
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
    <Text>reopened the issue</Text>
    <Text style="color: var(--color-foreground-level-5); margin-left: 8px;">
      {item.time_ago}
    </Text>
  </li>
{:else if item.variant === 'comment'}
  <li class={item.variant}>{item.variant}</li>
{/if}
