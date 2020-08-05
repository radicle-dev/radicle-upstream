<script>
  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import Label from "../Label.svelte";

  export let item = null;

  const icon = variant => {
    return {
      "open-issue": Icon.Issue,
      "reopen-issue": Icon.Issue,
      label: Icon.Label,
      assign: Icon.Member,
      "close-issue": Icon.CloseIssue,
    }[variant];
  };

  const action = variant => {
    return {
      "open-issue": "opened the issue",
      "reopen-issue": "closed the issue",
      label: "added the label",
      assign: "assigned",
      "close-issue": "reopened the issue",
    }[variant];
  };
</script>

<style>
  .labels {
    display: flex;
    margin-left: 4px;
  }
</style>

<svelte:component this={icon(item.variant)} />
<Avatar
  style="margin: 0 8px; height: 24px;"
  size="small"
  variant="circle"
  imageUrl={item.user.avatar_url}
  title={item.user.handle} />
<p style="color: var(--color-foreground-level-6);">{action(item.variant)}</p>
{#if item.variant === 'label'}
  <div class="labels">
    {#each item.labels as label}
      <Label
        style="margin-left: 4px;"
        title={label.title}
        color={label.color} />
    {/each}
  </div>
{:else if item.variant === 'assign'}
  <Avatar
    style="margin-left: 8px; height: 24px;"
    size="small"
    variant="circle"
    imageUrl={item.assignee.avatar_url}
    title={item.assignee.handle} />
{/if}
<p style="color: var(--color-foreground-level-5); margin-left: 8px;">
  {item.time_ago}
</p>
