<script>
  import { Icon, Title, Text } from "../../DesignSystem/Primitive";
</script>

<style>
  .container {
    max-width: 71.25rem;
    margin: 0 auto;
    padding: 32px 0;
    min-width: 500px;
  }

  li {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 85px;
    flex: 1;
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    padding: 16px;
  }

  li:hover {
    background-color: var(--color-foreground-level-1);
  }

  li:last-child {
    border-bottom: 0;
  }
  .title {
    display: flex;
  }

  .meta {
    display: flex;
    align-items: center;
  }
  .comment-count {
    display: flex;
    margin-right: 24px;
  }
</style>

<div class="container">
  <ul>
    <li>
      <div class="title">
        <Icon.Issue style="margin-right: 12px" />
        <div>
          <Title>Split server into CentralChain, and ChainApi</Title>
          <Text
            style="color: var(--color-foreground-level-5); padding-top: 6px">
            Opened 14 days ago by juliendonck
          </Text>
        </div>
      </div>
      <div class="meta">
        <div class="comment-count">
          <Icon.Replies />
          <Text
            style="margin-left: 4px;font-family: var(--typeface-bold); color:
            var(--color-foreground-level-5);">
            12
          </Text>
        </div>
        <Text style="color: var(--color-foreground-level-5);">
          Updatex 12 hours ago
        </Text>
      </div>
    </li>
    <li />
  </ul>
</div>
