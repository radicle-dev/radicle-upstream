<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script>
  import Avatar from "ui/DesignSystem/Avatar.svelte";
  import Icon from "ui/DesignSystem/Icon";
  import Label from "ui/DesignSystem/Label.svelte";

  export let item = null;

  const icon = variant => {
    return {
      "open-issue": Icon.ExclamationCircle,
      "reopen-issue": Icon.ExclamationCircle,
      label: Icon.Label,
      assign: Icon.User,
      "close-issue": Icon.CrossCircle,
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
{#if item.variant === "label"}
  <div class="labels">
    {#each item.labels as label}
      <Label
        style="margin-left: 4px;"
        title={label.title}
        color={label.color} />
    {/each}
  </div>
{:else if item.variant === "assign"}
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
