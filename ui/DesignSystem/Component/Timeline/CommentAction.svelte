<script>
  import { Avatar, Button, Text, Input } from "../../../DesignSystem/Primitive";

  export let user = null;
</script>

<style>
  .comment {
    padding-left: 0;
    display: flex;
  }
  section {
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

<div class="comment">
  <Avatar
    style="margin-right: 16px; height: 32px;"
    size="regular"
    variant="circle"
    imageUrl={user.avatar_url} />
  <section>
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
  </section>
</div>
