<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script>
  import Avatar from "ui/DesignSystem/Avatar.svelte";
  import Markdown from "ui/DesignSystem/Markdown.svelte";

  export let item = null;
</script>

<style>
  .comment-box {
    flex: 1;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
  }
  .meta {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  .content {
    padding: 1rem 1rem 0.375rem 1rem;
  }
</style>

<Avatar
  style="margin-right: 16px; height: 32px;"
  size="regular"
  variant="circle"
  imageUrl={item.user.avatar_url} />
<div class="comment-box">
  <div class="meta">
    <p class="typo-text-bold" style="margin-right: 8px;">{item.user.handle}</p>
    <p style="color: var(--color-foreground-level-6);">commented</p>
    <p style="color: var(--color-foreground-level-5); margin-left: 8px;">
      {item.time_ago}
    </p>
  </div>
  <div class="content">
    <Markdown content={item.comment} />
  </div>
</div>
